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
#[post("/admin/logout")]
pub async fn log_out(session: TypedSession) -> Result<HttpResponse, actix_web::Error> {
    if session.get_user_id().map_err(crate::utils::e500)?.is_none() {
        Ok(see_other("/login"))
    } else {
        session.log_out();
        FlashMessage::info("You have successfully logged out.").send();
        Ok(see_other("/login"))
    }
}
