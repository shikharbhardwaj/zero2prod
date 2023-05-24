use crate::{
    authentication::UserId,
    domain::SubscriberEmail,
    email_client::EmailClient,
    utils::{e500, see_other},
};
use actix_web::{post, web, HttpResponse};
use actix_web_flash_messages::FlashMessage;
use anyhow::Context;
use sqlx::PgPool;
use utoipa::ToSchema;

#[derive(serde::Deserialize, ToSchema)]
pub struct NewsletterRequestBody {
    title: String,
    text_content: String,
    html_content: String,
}

#[utoipa::path(
    request_body(content=NewsletterRequestBody, description="Publish newsletter", content_type="application/x-www-form-urlencoded"),
    responses(
        (status = 200, description = "OK"),
        (status = 303, description = "Login redirect"),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Server error"),
    ),
    tag = "zero2prod"
)]
#[post("/newsletters")]
#[tracing::instrument(name = "Publish newsletter", skip(form, pool, email_client))]
pub async fn publish_newsletter(
    form: web::Form<NewsletterRequestBody>,
    pool: web::Data<PgPool>,
    email_client: web::Data<EmailClient>,
    user_id: web::ReqData<UserId>,
) -> Result<HttpResponse, actix_web::Error> {
    let subscribers = get_confirmed_subscribers(&pool).await.map_err(e500)?;

    for subscriber in subscribers {
        match subscriber {
            Ok(subscriber) => {
                email_client
                    .send_mail(
                        &subscriber.email,
                        &form.title,
                        &form.html_content,
                        &form.text_content,
                    )
                    .await
                    .with_context(|| {
                        format!("Failed to send newsletter issue to {:?}", &subscriber.email)
                    })
                    .map_err(e500)?;
            }
            Err(error) => {
                tracing::warn!(error.cause_chain = ?error, "Skipping a confirmed subscriber.  \
            Their stored contact details are invalid.")
            }
        }
    }

    FlashMessage::info("The newsletter issue has been published!").send();
    Ok(see_other("/admin/newsletters"))
}

struct ConfirmedSubscriber {
    email: SubscriberEmail,
}

#[tracing::instrument(name = "Get confirmed subscribers from DB", skip(pool))]
async fn get_confirmed_subscribers(
    pool: &PgPool,
) -> Result<Vec<Result<ConfirmedSubscriber, anyhow::Error>>, anyhow::Error> {
    let rows = sqlx::query!(
        r#"
        SELECT email
        FROM subscriptions
        WHERE status = 'confirmed'
        "#,
    )
    .fetch_all(pool)
    .await?;

    let confirmed_subscribers = rows
        .into_iter()
        .map(|r| match SubscriberEmail::parse(r.email) {
            Ok(email) => Ok(ConfirmedSubscriber { email }),
            Err(error) => Err(anyhow::anyhow!(error)),
        })
        .collect();

    Ok(confirmed_subscribers)
}
