use actix_web::{get, http::header::ContentType, HttpResponse};
use actix_web_flash_messages::{IncomingFlashMessages, Level};
use askama::Template;
use std::fmt::Write;

use crate::templates::SendNewsletterTemplate;

#[get("/newsletters")]
pub async fn newsletter_issue_form(
    flash_messages: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    let mut error = String::new();

    for m in flash_messages.iter().filter(|m| m.level() == Level::Error) {
        writeln!(error, "{}", m.content()).unwrap();
    }

    let mut info = String::new();

    for m in flash_messages.iter().filter(|m| m.level() == Level::Info) {
        writeln!(info, "{}", m.content()).unwrap();
    }

    let html = SendNewsletterTemplate {
        error: &error,
        info: &info,
    }
    .render()
    .expect("Could not render send newsletter admin template.");

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}
