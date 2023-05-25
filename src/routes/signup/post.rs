use actix_web::{post, web, HttpResponse};
use actix_web_flash_messages::FlashMessage;
use anyhow::Context;
use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::authentication::{compute_password_hash, Credentials};
use crate::configuration::SignupSettings;
use crate::session_state::TypedSession;
use crate::telemetry::spawn_blocking_with_tracing;
use crate::utils::see_other;

#[derive(serde::Deserialize, ToSchema)]
pub struct SignupFormData {
    #[schema(value_type = String)]
    signup_token: Secret<String>,

    username: String,

    #[schema(value_type = String)]
    password: Secret<String>,
}

#[utoipa::path(
    request_body(content=SignupFormData, description="Signup", content_type="application/x-www-form-urlencoded"),
    responses(
        (status = 303, description = "Login redirect"),
        (status = 401, description = "Unauthorized"),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Server error"),
    ),
    tag = "zero2prod"
)]
#[post("/signup")]
#[tracing::instrument(skip(form, pool, session, signup_settings))]
pub async fn signup(
    form: web::Form<SignupFormData>,
    pool: web::Data<PgPool>,
    session: TypedSession,
    signup_settings: web::Data<SignupSettings>,
) -> Result<HttpResponse, actix_web::Error> {
    if !signup_settings.enabled {
        FlashMessage::error("Signups are disabled").send();
        return Ok(see_other("/login"));
    }

    if signup_settings.token.expose_secret() != form.0.signup_token.expose_secret() {
        FlashMessage::error("Invalid signup token").send();
        return Ok(see_other("/signup"));
    }

    let credentials = Credentials {
        username: form.0.username,
        password: form.0.password,
    };

    match insert_user(credentials.username, credentials.password, &pool).await {
        Ok(_) => {
            session.renew();
            FlashMessage::info("Successful signup!").send();
            Ok(see_other("/login"))
        }
        Err(e) => {
            FlashMessage::error(format!("Signup failed. Error: {:?}", e)).send();
            Ok(see_other("/signup"))
        }
    }
}

#[tracing::instrument(name = "Insert user details in DB", skip(pool, password))]
async fn insert_user(
    username: String,
    password: Secret<String>,
    pool: &PgPool,
) -> anyhow::Result<()> {
    let password_hash = spawn_blocking_with_tracing(move || compute_password_hash(password))
        .await?
        .context("Failed to hash password")?;

    sqlx::query!(
        r#"
        INSERT INTO users (user_id, username, password)
        VALUES (
            $1,
            $2,
            $3
        )"#,
        Uuid::new_v4(),
        username,
        password_hash.expose_secret()
    )
    .execute(pool)
    .await?;

    Ok(())
}
