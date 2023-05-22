use crate::helpers::{assert_is_redirect_to, spawn_app};

#[tokio::test]
async fn user_must_be_logged_in_to_access_change_password_form() {
    let app = spawn_app().await;

    let response = app.get_change_password().await;

    assert_is_redirect_to(&response, "/login");
}

#[tokio::test]
async fn user_must_be_logged_in_to_change_their_password() {
    let app = spawn_app().await;

    let body = serde_json::json!({
        "current_password": "secret",
        "new_password": "supersecret",
        "new_password_check": "supersecret",
    });

    let response = app.post_change_password(&body).await;

    assert_is_redirect_to(&response, "/login");
}
