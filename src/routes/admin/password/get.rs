use actix_web::{get, http::header::ContentType, HttpResponse};
use actix_web_flash_messages::{IncomingFlashMessages, Level};
use askama::Template;
use std::fmt::Write;

use crate::{
    session_state::TypedSession,
    templates::ChangePasswordTemplate,
    utils::{e500, see_other},
};

#[get("/admin/password")]
pub async fn change_password_form(
    session: TypedSession,
    flash_messages: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    if session.get_user_id().map_err(e500)?.is_none() {
        return Ok(see_other("/login"));
    }

    let mut error = String::new();

    for m in flash_messages.iter().filter(|m| m.level() == Level::Error) {
        writeln!(error, "{}", m.content()).unwrap();
    }

    let html = ChangePasswordTemplate { error: &error }
        .render()
        .expect("Could not render admin dashboard template.");

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}
