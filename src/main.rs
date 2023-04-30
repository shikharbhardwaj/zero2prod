use std::net::TcpListener;

use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;

use zero2prod::{
    configuration::get_configuration,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    tracing::info!("Starting application");

    let configuration = get_configuration().expect("Failed to read configuration.");
    let addr = configuration.application.get_listen_addr();

    let listener = TcpListener::bind(&addr).expect("Failed to bind to a local port.");
    tracing::info!("Listening on: {}", &addr);

    let connection_string = configuration.database.connection_string();
    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy(connection_string.expose_secret())
        .expect("Failed to connect to Postgres.");

    run(listener, connection_pool)?.await
}
