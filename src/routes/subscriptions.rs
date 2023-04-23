use chrono::Utc;
use uuid::Uuid;

use actix_web::{post, web, HttpResponse, Responder};
use sqlx::PgPool;

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
async fn subscribe(
    web::Form(form): web::Form<SubscribeRequest>,
    connection: web::Data<PgPool>,
) -> impl Responder {
    log::debug!("Received subscription request: {:?}", form);
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(connection.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Created(),
        Err(e) => {
            log::error!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError()
        }
    }
}
