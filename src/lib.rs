use std::net::TcpListener;

use actix_web::{get, HttpServer, Responder, HttpResponse, dev::Server, App};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

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

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error>{

    #[derive(OpenApi)]
    #[openapi(
        paths(
            health_check
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
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()))

    })
    .listen(listener)?
    .run();

    Ok(server)
}
