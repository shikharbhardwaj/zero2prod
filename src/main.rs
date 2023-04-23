use std::net::TcpListener;

use zero2prod::{configuration::get_configuration, startup::run};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let configuration = get_configuration().expect("Failed to read configuration.");

    let addr = format!("127.0.0.1:{}", configuration.application_port);

    let listener = TcpListener::bind(&addr).expect("Failed to bind to a local port.");
    log::info!("Listening on: {}", &addr);

    run(listener)?.await
}
