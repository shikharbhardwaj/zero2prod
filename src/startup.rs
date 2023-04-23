use std::net::TcpListener;

use actix_web::{dev::Server, App, HttpServer};
use utoipa::{OpenApi};
use utoipa_swagger_ui::SwaggerUi;

use crate::{models, routes::health_check, routes::subscribe};


pub fn run(listener: TcpListener) -> Result<Server, std::io::Error>{

    #[derive(OpenApi)]
    #[openapi(
        paths(
            crate::routes::health_check,
            crate::routes::subscribe
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