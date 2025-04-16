use crate::helper::spawn_app;

use reqwest::Client;

#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await;
    let client = Client::new();

    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("failed to make request");

    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}
