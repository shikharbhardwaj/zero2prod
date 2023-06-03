use crate::helpers::{assert_is_redirect_to, spawn_app};

#[tokio::test]
async fn an_error_flash_message_is_set_on_failure() {
    let app = spawn_app().await;
    let login_body = serde_json::json!({
        "username": "random-username",
        "password": "random-password",
    });

    let response = app.post_login(&login_body).await;

    assert_is_redirect_to(&response, "/login");

    let html_page = app.get_login_html().await;
    assert!(html_page.contains(r#"Invalid username or password"#));

    // The message should dissappear on reload.
    let html_page = app.get_login_html().await;
    assert!(!html_page.contains(r#"Invalid username or password"#));
}

#[tokio::test]
async fn redirect_to_admin_dashboard_on_login() {
    let app = spawn_app().await;

    let login_body = serde_json::json!({
        "username": app.user.username,
        "password": app.user.password,
    });

    let response = app.post_login(&login_body).await;
    assert_is_redirect_to(&response, "/admin/dashboard");

    let html_page = app.get_admin_dashboard_html().await;
    assert!(html_page.contains(&format!("Welcome {}", app.user.username)))
}
