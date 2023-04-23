use actix_web::{post, web, HttpResponse, Responder};

use crate::models::SubscribeRequest;

#[utoipa::path(
    request_body(content=SubscribeRequest, description="Details for subscription", content_type="application/x-www-form-urlencoded"),
    responses(
        (status = 201, description = "Subscribed successfully"),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Server error"),
    ),
    tag = "zero2prod"
)]
#[post("/subscribe")]
async fn subscribe(web::Form(form): web::Form<SubscribeRequest>) -> impl Responder {
    log::debug!("Received subscription request: {:?}", form);
    HttpResponse::Created()
}
