use actix_web::{put, web, HttpResponse, Responder};
use sqlx::PgPool;
use utoipa::IntoParams;
use uuid::Uuid;

#[derive(serde::Deserialize, IntoParams)]
pub struct Parameters {
    subscription_token: String,
}

#[utoipa::path(
    responses(
        (status = 200, description = "Subscription confirmed"),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Server error"),
    ),
    params(Parameters),
    tag = "zero2prod"
)]
#[put("/subscriptions/confirm")]
#[tracing::instrument(name = "Confirming a pending subscription", skip(parameters, pool))]
async fn confirm(parameters: web::Query<Parameters>, pool: web::Data<PgPool>) -> impl Responder {
    let id = match get_subscriber_id_from_token(&pool, &parameters.subscription_token).await {
        Ok(id) => id,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    match id {
        Some(subscriber_id) => {
            if confirm_subscriber(&pool, subscriber_id).await.is_err() {
                return HttpResponse::InternalServerError().finish();
            }
            HttpResponse::Ok().finish()
        }
        _ => HttpResponse::Unauthorized().finish(),
    }
}

#[tracing::instrument(
    name = "Update subscriber confirmation details in DB",
    skip(pool, subscriber_id)
)]
async fn confirm_subscriber(pool: &PgPool, subscriber_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"UPDATE subscriptions SET status = 'confirmed' WHERE id = $1"#,
        subscriber_id
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    Ok(())
}

#[tracing::instrument(name = "Get subscriber_id from token", skip(subscription_token, pool))]
pub async fn get_subscriber_id_from_token(
    pool: &PgPool,
    subscription_token: &str,
) -> Result<Option<Uuid>, sqlx::Error> {
    let result = sqlx::query!(
        "SELECT subscriber_id FROM subscription_tokens \
WHERE subscription_token = $1",
        subscription_token,
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(result.map(|r| r.subscriber_id))
}
