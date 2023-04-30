use std::net::TcpListener;

use sqlx::PgPool;

use zero2prod::{
    configuration::get_configuration,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into());
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");

    let addr = format!("127.0.0.1:{}", configuration.application_port);

    let listener = TcpListener::bind(&addr).expect("Failed to bind to a local port.");
    tracing::info!("Listening on: {}", &addr);

    let connection_string = configuration.database.connection_string();
    let connection_pool = PgPool::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.");

    run(listener, connection_pool)?.await
}
