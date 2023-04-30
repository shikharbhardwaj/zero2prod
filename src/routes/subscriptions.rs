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
#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, connection),
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
async fn subscribe(
    web::Form(form): web::Form<SubscribeRequest>,
    connection: web::Data<PgPool>,
) -> impl Responder {
    match insert_subscriber(&form, &connection).await {
        Ok(_) => HttpResponse::Created(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database.",
    skip(req, connection)
)]
async fn insert_subscriber(req: &SubscribeRequest, connection: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        req.email,
        req.name,
        Utc::now()
    )
    .execute(connection)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    Ok(())
}
