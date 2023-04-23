use std::net::TcpListener;

use zero2prod::startup::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let addr = "127.0.0.1:8000";

    let listener = TcpListener::bind(addr).expect("Failed to bind to a local port.");
    log::info!("Listening on: {}", addr);

    run(listener)?.await
}
