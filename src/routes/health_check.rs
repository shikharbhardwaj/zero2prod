use actix_web::{get, HttpResponse, Responder};

#[utoipa::path(
    responses(
        (status = 200, description = "Application healthy"),
    ),
    tag = "zero2prod"
)]
#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}
