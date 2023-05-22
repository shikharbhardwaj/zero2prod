use actix_web::{post, web, HttpResponse};
use secrecy::Secret;

use crate::{
    session_state::TypedSession,
    utils::{e500, see_other},
};

#[derive(serde::Deserialize)]
pub struct FormData {
    _current_password: Secret<String>,
    _new_password: Secret<String>,
    _new_password_check: Secret<String>,
}

#[utoipa::path(
    request_body(content=SubscriptionRequest, description="Details for subscription", content_type="application/x-www-form-urlencoded"),
    responses(
        (status = 200, description = "OK"),
        (status = 401, description = "Unauthorized"),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Server error"),
    ),
    tag = "zero2prod"
)]
#[post("/admin/password")]
#[tracing::instrument(name = "Changing password", skip(_form, session))]
pub async fn change_password(
    _form: web::Form<FormData>,
    session: TypedSession,
) -> Result<HttpResponse, actix_web::Error> {
    if session.get_user_id().map_err(e500)?.is_none() {
        return Ok(see_other("/login"));
    }

    todo!()
}
