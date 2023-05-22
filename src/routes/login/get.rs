use actix_web::{get, http::header::ContentType, HttpResponse};
use actix_web_flash_messages::{IncomingFlashMessages, Level};
use askama::Template;
use std::fmt::Write;

use crate::templates::LoginTemplate;

#[get("/login")]
pub async fn login_form(flash_messages: IncomingFlashMessages) -> HttpResponse {
    let mut error = String::new();

    for m in flash_messages.iter().filter(|m| m.level() == Level::Error) {
        writeln!(error, "{}", m.content()).unwrap();
    }

    let mut info = String::new();

    for m in flash_messages.iter().filter(|m| m.level() == Level::Info) {
        writeln!(info, "{}", m.content()).unwrap();
    }

    let html = LoginTemplate {
        error: &error,
        info: &info,
    }
    .render()
    .expect("Could not render login template.");

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html)
}
