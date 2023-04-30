use chrono::Utc;
use tracing::Instrument;
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
    let request_id = Uuid::new_v4();

    let request_span = tracing::info_span!("Received subscription request",
        %request_id,
        subscriber_email = %form.email,
        subscriber_name = %form.name);

    let _request_span_guard = request_span.enter();

    let query_span = tracing::info_span!("Saving new subscriber details in the database");

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
    .instrument(query_span)
    .await
    {
        Ok(_) => {
            tracing::info!("Successfully processed subscription request: {:?}", form);
            HttpResponse::Created()
        }
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError()
        }
    }
}
