use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{domain, routes::health_check, routes::subscribe};

pub fn run(listener: TcpListener, connection: PgPool) -> Result<Server, std::io::Error> {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            crate::routes::health_check,
            crate::routes::subscribe
        ),
        components(
            schemas(domain::SubscriptionRequest),
            schemas(domain::SubscriberName),
            schemas(domain::SubscriberEmail),
        ),
        tags(
            (name = "zero2prod", description = "Newsletter app built following the Rust: Zero to Production book.")
        )
    )]
    struct ApiDoc;

    let openapi = ApiDoc::openapi();

    let connection = web::Data::new(connection);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .service(health_check)
            .service(subscribe)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
