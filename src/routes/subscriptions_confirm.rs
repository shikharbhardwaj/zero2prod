use actix_web::{put, web, HttpResponse, Responder};
use sqlx::PgPool;
use utoipa::IntoParams;

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
#[tracing::instrument(
    name = "Confirming a pending subscription",
    skip(parameters, _connection)
)]
async fn confirm(
    parameters: web::Query<Parameters>,
    _connection: web::Data<PgPool>,
) -> impl Responder {
    log::info!("Got request for token: {}", parameters.0.subscription_token);
    HttpResponse::Ok().finish()
}
