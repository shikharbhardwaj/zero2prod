use actix_web::{post, web, HttpResponse, Responder};
use sqlx::PgPool;
use utoipa::IntoParams;

#[derive(serde::Deserialize, IntoParams)]
pub struct Parameters {
    _subscription_token: String,
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
#[post("/subscriptions/confirm")]
#[tracing::instrument(
    name = "Confirming a pending subscription",
    skip(_parameters, _connection)
)]
async fn confirm(
    _parameters: web::Query<Parameters>,
    _connection: web::Data<PgPool>,
) -> impl Responder {
    HttpResponse::Ok().finish()
}
