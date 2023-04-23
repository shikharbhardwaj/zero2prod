use std::net::TcpListener;

use actix_web::{get, HttpServer, Responder, HttpResponse, dev::Server, App, post, web};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::models::SubscribeRequest;

mod models;

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


#[utoipa::path(
    request_body(content=SubscribeRequest, description="Details for subscription", content_type="application/x-www-form-urlencoded"),
    responses(
        (status = 201, description = "Subscribed successfully"),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Server error"),
    ),
    tag = "zero2prod"
)]
#[post("/subscribe")]
async fn subscribe(web::Form(form): web::Form<SubscribeRequest>) -> impl Responder {
    log::debug!("Received subscription request: {:?}", form);
    HttpResponse::Created()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error>{

    #[derive(OpenApi)]
    #[openapi(
        paths(
            health_check,
            subscribe
        ),
        components(
            schemas(models::SubscribeRequest),
        ),
        tags(
            (name = "zero2prod", description = "Newsletter app built following the Rust: Zero to Production book.")
        )
    )]
    struct ApiDoc;

    let openapi = ApiDoc::openapi();

    let server = HttpServer::new(move || {
        App::new()
            .service(health_check)
            .service(subscribe)
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()))

    })
    .listen(listener)?
    .run();

    Ok(server)
}
