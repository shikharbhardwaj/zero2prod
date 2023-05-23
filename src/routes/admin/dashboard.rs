use actix_web::{get, http::header::ContentType, web, HttpResponse};
use anyhow::Context;
use askama::Template;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{authentication::UserId, templates::AdminDashboardTemplate, utils::e500};

#[get("/dashboard")]
pub async fn admin_dashboard(
    user_id: web::ReqData<UserId>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = user_id.into_inner();

    let username = get_username(*user_id, &pool).await.map_err(e500)?;

    let html = AdminDashboardTemplate {
        username: &username,
    }
    .render()
    .expect("Could not render admin dashboard template.");

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}

#[tracing::instrument(name = "Get username", skip(pool))]
pub async fn get_username(user_id: Uuid, pool: &PgPool) -> Result<String, anyhow::Error> {
    let row = sqlx::query!(
        r#"
        SELECT username
        FROM users
        WHERE user_id = $1
        "#,
        user_id,
    )
    .fetch_one(pool)
    .await
    .context("Failed to perform a query to retrieve a username.")?;
    Ok(row.username)
}
