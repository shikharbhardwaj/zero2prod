use actix_session::Session;
use actix_web::{get, http::header::ContentType, web, HttpResponse};
use anyhow::Context;
use reqwest::header::LOCATION;
use sqlx::PgPool;
use uuid::Uuid;

#[get("/admin/dashboard")]
pub async fn admin_dashboard(
    session: Session,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let username = if let Some(user_id) = session.get::<Uuid>("user_id").map_err(|_| {
        actix_web::error::ErrorInternalServerError(anyhow::anyhow!(
            "Failed to get user id from session"
        ))
    })? {
        get_username(user_id, &pool).await.map_err(|_| {
            actix_web::error::ErrorInternalServerError(anyhow::anyhow!("Failed to get user name"))
        })?
    } else {
        return Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, "/login"))
            .finish());
    };

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"<!DOCTYPE html>
            <html lang="en">
                <head>
                    <meta http-equiv="content-type" content="text/html; charset=utf-8">
                    <title>Admin dashboard</title>
                </head>
                <body>
                    <p>Welcome {username}!</p>
                </body>
            </html>"#
        )))
}

#[tracing::instrument(name = "Get username", skip(pool))]
async fn get_username(user_id: Uuid, pool: &PgPool) -> Result<String, anyhow::Error> {
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
