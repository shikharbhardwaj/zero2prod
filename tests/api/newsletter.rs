use std::time::Duration;

use crate::helpers::{assert_is_redirect_to, spawn_app, ConfirmationLinks, TestApp};
use fake::faker::internet::en::SafeEmail;
use fake::faker::name::en::Name;
use fake::Fake;
use wiremock::matchers::{any, method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn newsletter_are_not_delivered_to_unconfirmed_subscribers() {
    // Arrange
    let app = spawn_app().await;
    create_unconfirmed_subscriber(&app).await;

    Mock::given(any())
        .respond_with(ResponseTemplate::new(200))
        .expect(0)
        .mount(&app.email_server)
        .await;

    // Act

    // A sketch of the newsletter payload structure.
    let newsletter_request_body = serde_json::json!({
        "title": "Newsletter title",
        "text_content": "Newsletter body as plain text",
        "html_content": "<p>Newsletter body as HTML</p>",
        "idempotency_key": uuid::Uuid::new_v4().to_string(),
    });

    app.user.login(&app).await;

    let response = app.post_newsletters(&newsletter_request_body).await;
    assert_is_redirect_to(&response, "/admin/newsletters");

    let html_page = app.get_newsletter_publish_html().await;
    assert!(html_page.contains(
        "The newsletter issue has been accepted - \
                                emails will go out shortly."
    ));
    app.dispatch_all_pending_deliveries().await;
}

#[tokio::test]
async fn newsletters_are_delivered_to_confirmed_subscribers() {
    let app = spawn_app().await;

    app.user.login(&app).await;

    create_confirmed_subscriber(&app).await;

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    // Act
    let newsletter_request_body = serde_json::json!({
        "title": "Newsletter title",
        "text_content": "Newsletter body as plain text",
        "html_content": "<p>Newsletter body as HTML</p>",
        "idempotency_key": uuid::Uuid::new_v4().to_string(),
    });

    let response = app.post_newsletters(&newsletter_request_body).await;
    assert_is_redirect_to(&response, "/admin/newsletters");

    let html_page = app.get_newsletter_publish_html().await;
    assert!(html_page.contains(
        "The newsletter issue has been accepted - \
                                emails will go out shortly."
    ));
    app.dispatch_all_pending_deliveries().await;
}

#[tokio::test]
async fn newsletter_returns_400_for_invalid_data() {
    // Arrange
    let app = spawn_app().await;

    app.user.login(&app).await;

    let test_cases = vec![
        (
            serde_json::json!({
                "text_content": "Some text",
                "html_content": "<p>Some html</p>"
            }),
            "missing_title",
        ),
        (
            serde_json::json!({
                "title": "Newsletter!"
            }),
            "Missing content",
        ),
    ];

    for (invalid_body, error_msg) in test_cases {
        let response = app.post_newsletters(&invalid_body).await;

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_msg
        );
    }
}

#[tokio::test]
async fn newsletter_request_without_auth_gets_rejected() {
    let app = spawn_app().await;

    let body = serde_json::json!({
        "context_text": "Some text",
        "content_html": "Some html",
        "title": "Some title"
    });

    let response = app.post_newsletters(&body).await;

    assert_is_redirect_to(&response, "/login");
}

#[tokio::test]
async fn newsletter_creation_is_idempotent() {
    let app = spawn_app().await;
    create_confirmed_subscriber(&app).await;

    app.user.login(&app).await;

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    let newsletter_request_body = serde_json::json!({
        "title": "Newsletter title",
        "text_content": "Newsletter body as plain text",
        "html_content": "<p>Newsletter body as HTML</p>",
        "idempotency_key": uuid::Uuid::new_v4().to_string(),
    });

    let response = app.post_newsletters(&newsletter_request_body).await;
    assert_is_redirect_to(&response, "/admin/newsletters");

    let html_page = app.get_newsletter_publish_html().await;
    assert!(html_page.contains(
        "The newsletter issue has been accepted - \
                                emails will go out shortly."
    ));

    // Repeat again, expecting the same user facing behaviour but the emails
    // should only be sent once.
    let response = app.post_newsletters(&newsletter_request_body).await;
    assert_is_redirect_to(&response, "/admin/newsletters");

    let html_page = app.get_newsletter_publish_html().await;
    assert!(html_page.contains(
        "The newsletter issue has been accepted - \
                                emails will go out shortly."
    ));
    app.dispatch_all_pending_deliveries().await;
}

#[tokio::test]
async fn concurrent_form_submission_is_handled_gracefully() {
    let app = spawn_app().await;
    create_confirmed_subscriber(&app).await;
    app.user.login(&app).await;

    Mock::given(path("/email"))
        .and(method("POST"))
        // Setting a long delay to ensure second request arrives before the
        // first one completes.
        .respond_with(ResponseTemplate::new(200).set_delay(Duration::from_secs(2)))
        .expect(1)
        .mount(&app.email_server)
        .await;

    let newsletter_request_body = serde_json::json!({
        "title": "Newsletter title",
        "text_content": "Newsletter body as plain text",
        "html_content": "<p>Newsletter body as HTML</p>",
        "idempotency_key": uuid::Uuid::new_v4().to_string(),
    });

    let response1 = app.post_newsletters(&newsletter_request_body);
    let response2 = app.post_newsletters(&newsletter_request_body);

    let (response1, response2) = tokio::join!(response1, response2);

    assert_eq!(response1.status(), response2.status());
    assert_eq!(
        response1.text().await.unwrap(),
        response2.text().await.unwrap()
    );
    app.dispatch_all_pending_deliveries().await;
}

async fn create_unconfirmed_subscriber(app: &TestApp) -> ConfirmationLinks {
    let name: String = Name().fake();
    let email: String = SafeEmail().fake();

    let body = serde_urlencoded::to_string(serde_json::json!({
        "name": name,
        "email": email
    }))
    .unwrap();

    let _mock_guard = Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .named("Create unconfirmed subscriber")
        .expect(1)
        .mount_as_scoped(&app.email_server)
        .await;

    app.post_subscriptions(body)
        .await
        .error_for_status()
        .unwrap();

    let email_request = &app
        .email_server
        .received_requests()
        .await
        .unwrap()
        .pop()
        .unwrap();
    app.get_confirmation_links(email_request)
}

async fn create_confirmed_subscriber(app: &TestApp) {
    let confirmation_link = create_unconfirmed_subscriber(app).await;

    reqwest::get(confirmation_link.html)
        .await
        .unwrap()
        .error_for_status()
        .unwrap();
}
