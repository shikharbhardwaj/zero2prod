use chrono::Utc;
use uuid::Uuid;

use actix_web::{post, web, HttpResponse, Responder};
use sqlx::PgPool;

use crate::domain::{SubscriberEmail, SubscriberName, SubscriptionRequest};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

#[utoipa::path(
    request_body(content=SubscriptionRequest, description="Details for subscription", content_type="application/x-www-form-urlencoded"),
    responses(
        (status = 201, description = "Subscribed successfully"),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Server error"),
    ),
    tag = "zero2prod"
)]
#[post("/subscriptions")]
#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, connection),
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
async fn subscribe(form: web::Form<FormData>, connection: web::Data<PgPool>) -> impl Responder {
    let name = match SubscriberName::parse(form.0.name) {
        Ok(name) => name,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    let email = match SubscriberEmail::parse(form.0.email) {
        Ok(email) => email,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    let subcription_request = SubscriptionRequest { email, name };

    match insert_subscriber(&subcription_request, &connection).await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database.",
    skip(req, connection)
)]
async fn insert_subscriber(
    req: &SubscriptionRequest,
    connection: &PgPool,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        req.email.as_ref(),
        req.name.as_ref(),
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
