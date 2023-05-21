use actix_web::http::header::LOCATION;
use actix_web::{post, web, HttpResponse};
use secrecy::Secret;
use utoipa::ToSchema;

#[derive(serde::Deserialize, ToSchema)]
pub struct LoginFormData {
    _username: String,
    _password: Secret<String>,
}

#[utoipa::path(
    request_body(content=LoginFormData, description="Login", content_type="application/x-www-form-urlencoded"),
    responses(
        (status = 201, description = "Subscribed successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Server error"),
    ),
    tag = "zero2prod"
)]
#[post("/login")]
pub async fn login(_form: web::Form<LoginFormData>) -> HttpResponse {
    HttpResponse::SeeOther()
        .insert_header((LOCATION, "/"))
        .finish()
}
