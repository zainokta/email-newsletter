mod common;

use common::spawn_app;

#[tokio::test]
async fn health_check_success() {
    let address = spawn_app().await;

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/healthz", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
}
