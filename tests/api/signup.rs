use secrecy::ExposeSecret;
use uuid::Uuid;

use crate::helpers::{assert_is_redirect_to, spawn_app};

#[tokio::test]
async fn signup_fails_with_invalid_token() {
    let app = spawn_app().await;

    let body = serde_json::json!({
        "username": "test",
        "password": Uuid::new_v4(),
        "signup_token": Uuid::new_v4(),
    });

    let response = app.post_signup(&body).await;

    assert_is_redirect_to(&response, "/signup");

    let html = app.get_signup_html().await;

    assert!(html.contains("Invalid signup token"));
}

#[tokio::test]
async fn signup_succeeds_with_valid_token() {
    let app = spawn_app().await;

    let body = serde_json::json!({
        "username": "test",
        "password": Uuid::new_v4(),
        "signup_token": app.signup_token.expose_secret(),
    });

    let response = app.post_signup(&body).await;

    assert_is_redirect_to(&response, "/login");

    let html = app.get_login_html().await;

    assert!(html.contains("Successful signup!"));
}
