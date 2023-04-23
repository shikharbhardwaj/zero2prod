use std::net::{TcpListener};



#[tokio::test]
async fn health_check_responds_ok() {
    let url = spawn_app();

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/health_check", &url))
        .send()
        .await
        .expect("Failed to execute health check request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to a local port.");
    let addr = listener.local_addr().expect("Could not get local address.");

    let server = zero2prod::run(listener).expect("Failed to run app.");
    let _ = tokio::spawn(server);

    format!("http://{}", addr.to_string())
}