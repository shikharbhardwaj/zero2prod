use actix_web::error::InternalError;
use actix_web::http::header::LOCATION;
use actix_web::{post, web, HttpResponse};
use actix_web_flash_messages::FlashMessage;
use secrecy::Secret;
use sqlx::PgPool;
use utoipa::ToSchema;

use crate::authentication::{validate_credentials, AuthError, Credentials};
use crate::routes::error_chain_fmt;
use crate::session_state::TypedSession;
use crate::utils::see_other;

#[derive(serde::Deserialize, ToSchema)]
pub struct LoginFormData {
    username: String,

    #[schema(value_type = String)]
    password: Secret<String>,
}

#[utoipa::path(
    request_body(content=LoginFormData, description="Login", content_type="application/x-www-form-urlencoded"),
    responses(
        (status = 303, description = "Login redirect"),
        (status = 401, description = "Unauthorized"),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Server error"),
    ),
    tag = "zero2prod"
)]
#[post("/login")]
#[tracing::instrument(
    skip(form, pool, session),
    fields(username=tracing::field::Empty, user_id=tracing::field::Empty)
)]
pub async fn login(
    form: web::Form<LoginFormData>,
    pool: web::Data<PgPool>,
    session: TypedSession,
) -> Result<HttpResponse, InternalError<LoginError>> {
    let credentials = Credentials {
        username: form.0.username,
        password: form.0.password,
    };
    tracing::Span::current().record("username", &tracing::field::display(&credentials.username));

    match validate_credentials(credentials, &pool).await {
        Ok(user_id) => {
            tracing::Span::current().record("user_id", &tracing::field::display(&user_id));
            session.renew();
            session
                .insert_user_id(user_id)
                .map_err(|e| login_redirect(LoginError::UnexpectedError(e.into())))?;
            Ok(see_other("/admin/dashboard"))
        }
        Err(e) => {
            let e = match e {
                AuthError::InvalidCredentials(_) => LoginError::AuthError(e.into()),
                AuthError::UnexpectedError(_) => LoginError::UnexpectedError(e.into()),
            };
            Err(login_redirect(e))
        }
    }
}

#[derive(thiserror::Error)]
pub enum LoginError {
    #[error("Invalid username or password")]
    AuthError(#[source] anyhow::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for LoginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

// Redirect to the login page with an error message.
fn login_redirect(e: LoginError) -> InternalError<LoginError> {
    FlashMessage::error(e.to_string()).send();
    let response = HttpResponse::SeeOther()
        .insert_header((LOCATION, "/login"))
        .finish();
    InternalError::from_response(e, response)
}
