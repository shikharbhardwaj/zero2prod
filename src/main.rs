use std::net::TcpListener;

use sqlx::postgres::PgPoolOptions;

use zero2prod::{
    configuration::get_configuration,
    email_client::EmailClient,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    tracing::info!("Starting newsletter API");

    let configuration = get_configuration().expect("Failed to read configuration.");
    let addr = configuration.application.get_listen_addr();

    let listener = TcpListener::bind(&addr).expect("Failed to bind to a local port.");
    tracing::info!("Listening on: {}", &addr);

    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.database.with_db());

    let sender_email = configuration
        .email_client
        .sender()
        .expect("Invalid sender email address.");
    let timeout = configuration.email_client.timeout();
    let email_client = EmailClient::new(
        configuration.email_client.base_url,
        sender_email,
        configuration.email_client.authorization_token,
        timeout,
    );

    run(listener, connection_pool, email_client)?.await
}
