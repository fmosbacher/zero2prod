use std::net::TcpListener;

use reqwest::Client;

type BaseUrl = String;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let url = spawn_app();
    let client = Client::new();

    // Act
    let response = client
        .get(format!("{url}/health-check"))
        .send()
        .await
        .expect("200 OK from health-check endpoint");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}

fn spawn_app() -> BaseUrl {
    let listener =
        TcpListener::bind("127.0.0.1:0").expect("listerner binded to random available port");
    let port = listener
        .local_addr()
        .expect("local socket address data")
        .port();
    let server = zero2prod::run(listener).expect("server running");
    tokio::spawn(server);
    format!("http://127.0.0.1:{port}")
}
