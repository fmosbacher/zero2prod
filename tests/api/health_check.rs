use reqwest::Client;

use crate::helpers::spawn_app;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let base_url = spawn_app();
    let client = Client::new();

    // Act
    let response = client
        .get(format!("{base_url}/health-check"))
        .send()
        .await
        .expect("200 OK from health-check endpoint");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}
