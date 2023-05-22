use actix_web::{get, http::header::ContentType, HttpResponse};
use askama::Template;

use crate::{
    session_state::TypedSession,
    templates::ChangePasswordTemplate,
    utils::{e500, see_other},
};

#[get("/admin/password")]
pub async fn change_password_form(session: TypedSession) -> Result<HttpResponse, actix_web::Error> {
    if session.get_user_id().map_err(e500)?.is_none() {
        return Ok(see_other("/login"));
    }

    let html = ChangePasswordTemplate { error: "" }
        .render()
        .expect("Could not render admin dashboard template.");

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}
