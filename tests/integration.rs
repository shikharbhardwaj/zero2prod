use std::net::TcpListener;



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

#[tokio::test]
async fn subsribe_returns_ok_for_valid_form() {
    let url = spawn_app();
    let client = reqwest::Client::new();

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscribe", &url))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(201, response.status().as_u16());
}


#[tokio::test]
async fn subsribe_returns_bad_request_for_missing_data() {
    let url = spawn_app();
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email")
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscribe", &url))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(400,
            response.status().as_u16(),
            "The API did  not fail with 400 bad request when the payload was
            {}.", error_message
        );
    }
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to a local port.");
    let addr = listener.local_addr().expect("Could not get local address.");

    let server = zero2prod::run(listener).expect("Failed to run app.");
    let _ = tokio::spawn(server);

    format!("http://{}", addr.to_string())
}