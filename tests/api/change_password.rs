use uuid::Uuid;

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

    let new_password = Uuid::new_v4().to_string();

    let body = serde_json::json!({
        "current_password": Uuid::new_v4().to_string(),
        "new_password": new_password,
        "new_password_check": new_password,
    });

    let response = app.post_change_password(&body).await;

    assert_is_redirect_to(&response, "/login");
}

#[tokio::test]
async fn change_password_fails_on_mismatched_new_passwords() {
    let app = spawn_app().await;

    let new_password = Uuid::new_v4().to_string();
    let new_password_check = Uuid::new_v4().to_string();

    app.post_login(&serde_json::json!({
        "username": &app.user.username,
        "password": &app.user.password,
    }))
    .await;

    let response = app
        .post_change_password(&serde_json::json!({
            "current_password": &app.user.password,
            "new_password": new_password,
            "new_password_check": new_password_check,
        }))
        .await;

    assert_is_redirect_to(&response, "/admin/password");

    let html_page = app.get_change_password_html().await;

    assert!(html_page.contains("The new passwords did not match."));
}

#[tokio::test]
async fn current_password_must_be_valid() {
    let app = spawn_app().await;

    let new_password = Uuid::new_v4().to_string();
    let wrong_password = Uuid::new_v4().to_string();

    app.post_login(&serde_json::json!({
        "username": &app.user.username,
        "password": &app.user.password,
    }))
    .await;

    let response = app
        .post_change_password(&serde_json::json!({
            "current_password": wrong_password,
            "new_password": new_password,
            "new_password_check": new_password,
        }))
        .await;

    assert_is_redirect_to(&response, "/admin/password");

    let html_page = app.get_change_password_html().await;

    assert!(html_page.contains("The current password is incorrect."));
}
