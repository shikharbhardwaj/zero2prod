use actix_web::{get, http::header::ContentType, HttpResponse};
use askama::Template;

use crate::templates::HomeTemplate;

#[get("/")]
pub async fn home() -> HttpResponse {
    let html = HomeTemplate {}
        .render()
        .expect("Could not render home template.");

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html)
}
