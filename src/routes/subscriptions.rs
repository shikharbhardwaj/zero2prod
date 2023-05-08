use chrono::Utc;
use uuid::Uuid;

use actix_web::{post, web, HttpResponse, Responder};
use sqlx::PgPool;

use crate::{
    domain::{SubscriberEmail, SubscriberName, SubscriptionRequest},
    email_client::EmailClient,
};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

impl TryFrom<FormData> for SubscriptionRequest {
    type Error = String;

    fn try_from(value: FormData) -> Result<Self, Self::Error> {
        let name = SubscriberName::parse(value.name)?;
        let email = SubscriberEmail::parse(value.email)?;
        Ok(Self { email, name })
    }
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
    skip(form, connection, email_client),
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
async fn subscribe(
    form: web::Form<FormData>,
    connection: web::Data<PgPool>,
    email_client: web::Data<EmailClient>,
) -> impl Responder {
    let subcription_request = match form.0.try_into() {
        Ok(subscription_request) => subscription_request,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    if insert_subscriber(&subcription_request, &connection)
        .await
        .is_err()
    {
        log::error!("Could not save subcriber data in database.");
        return HttpResponse::InternalServerError().finish();
    }

    if send_confirmation_email(&email_client, subcription_request)
        .await
        .is_err()
    {
        log::error!("Could not send confirmation email.");
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Created().finish()
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
        INSERT INTO subscriptions (id, email, name, subscribed_at, status)
        VALUES ($1, $2, $3, $4, 'pending_confirmation')
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

#[tracing::instrument(
    name = "Send a confirmation email to a new subscriber",
    skip(email_client, new_subscriber)
)]
async fn send_confirmation_email(
    email_client: &EmailClient,
    new_subscriber: SubscriptionRequest,
) -> Result<(), reqwest::Error> {
    let confirmation_link = "https://some-non-existent-domain.com/subscriptions/confirm";

    let html_body = &format!(
        "Welcome to our newsletter! <br />\
                Click <a href=\"{}\">here</a> to confirm your subscriptions.",
        confirmation_link
    );

    let plain_body = &format!(
        "Welcome to our newsletter!\nVisit {} to confirm your subscription.",
        confirmation_link
    );

    email_client
        .send_mail(new_subscriber.email, "Welcome!", html_body, plain_body)
        .await
}
