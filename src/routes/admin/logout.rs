use actix_web::{post, HttpResponse};
use actix_web_flash_messages::FlashMessage;

use crate::{session_state::TypedSession, utils::see_other};

#[utoipa::path(
    responses(
        (status = 303, description = "Logout redirect"),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Server error"),
    ),
    tag = "zero2prod"
)]
#[post("/logout")]
pub async fn log_out(session: TypedSession) -> Result<HttpResponse, actix_web::Error> {
    session.log_out();
    FlashMessage::info("You have successfully logged out.").send();
    Ok(see_other("/login"))
}
