use std::net::TcpListener;

use sqlx::PgPool;
use tracing::dispatcher::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, EnvFilter, Registry};
use zero2prod::{configuration::get_configuration, startup::run};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    LogTracer::init().expect("Failed to set logger");

    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let formatting_layer = BunyanFormattingLayer::new("zero2prod".into(), std::io::stdout);

    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);

    set_global_default(subscriber.into()).expect("Failed to set subscriber.");

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
