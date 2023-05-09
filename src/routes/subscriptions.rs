use chrono::Utc;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use uuid::Uuid;

use actix_web::{post, web, HttpResponse, Responder};
use sqlx::{PgPool, Postgres, Transaction};

use crate::{
    domain::{SubscriberEmail, SubscriberName, SubscriptionRequest},
    email_client::EmailClient,
    startup::ApplicationBaseUrl,
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
    skip(form, pool, email_client, base_url),
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
async fn subscribe(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
    email_client: web::Data<EmailClient>,
    base_url: web::Data<ApplicationBaseUrl>,
) -> impl Responder {
    let subcription_request = match form.0.try_into() {
        Ok(subscription_request) => subscription_request,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    let mut transaction = match pool.begin().await {
        Ok(transaction) => transaction,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let subscriber_id = match insert_subscriber(&mut transaction, &subcription_request).await {
        Ok(subscriber_id) => subscriber_id,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let subscription_token = generate_subscription_token();

    if store_token(&mut transaction, subscriber_id, &subscription_token)
        .await
        .is_err()
    {
        return HttpResponse::InternalServerError().finish();
    }

    if transaction.commit().await.is_err() {
        log::error!("Failed to commit DB transaction");
        return HttpResponse::InternalServerError().finish();
    }

    if send_confirmation_email(
        &email_client,
        subcription_request,
        &base_url.0,
        &subscription_token,
    )
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
    skip(transaction, req)
)]
async fn insert_subscriber(
    transaction: &mut Transaction<'_, Postgres>,
    req: &SubscriptionRequest,
) -> Result<Uuid, sqlx::Error> {
    let subscriber_id = Uuid::new_v4();

    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at, status)
        VALUES ($1, $2, $3, $4, 'pending_confirmation')
        "#,
        subscriber_id,
        req.email.as_ref(),
        req.name.as_ref(),
        Utc::now()
    )
    .execute(transaction)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    Ok(subscriber_id)
}

#[tracing::instrument(
    name = "Send a confirmation email to a new subscriber",
    skip(email_client, new_subscriber)
)]
async fn send_confirmation_email(
    email_client: &EmailClient,
    new_subscriber: SubscriptionRequest,
    base_url: &str,
    subscription_token: &str,
) -> Result<(), reqwest::Error> {
    let confirmation_link = format!(
        "{}/subscriptions/confirm?subscription_token={}",
        base_url, subscription_token
    );

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

fn generate_subscription_token() -> String {
    let mut rng = thread_rng();

    std::iter::repeat_with(|| rng.sample(Alphanumeric))
        .map(char::from)
        .take(25)
        .collect()
}

#[tracing::instrument(
    name = "Store subscription token in the database",
    skip(transaction, subscription_token)
)]
pub async fn store_token(
    transaction: &mut Transaction<'_, Postgres>,
    subscriber_id: Uuid,
    subscription_token: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"INSERT INTO subscription_tokens (subscription_token, subscriber_id)
        VALUES ($1, $2)"#,
        subscription_token,
        subscriber_id
    )
    .execute(transaction)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}
