use actix_web::{HttpResponse, post, web};
use utoipa::ToSchema;


#[derive(serde::Deserialize, ToSchema)]
pub struct NewsletterRequest {
    title: String,
    content: Content
}

#[derive(serde::Deserialize)]
pub struct Content {
    text: String,
    html: String
}

#[utoipa::path(
    request_body(content=NewsletterRequest, description="Details required to send the newsletter issue", content_type="application/json"),
    responses(
        (status = 200, description = "Emails sent"),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Server error"),
    ),
    tag = "zero2prod"
)]
#[post("/newsletters")]
#[tracing::instrument(name = "Confirming a pending subscription", skip(_body))]
pub async fn publish_newsletter(_body: web::Json<NewsletterRequest>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
