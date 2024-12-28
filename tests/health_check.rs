#[tokio::test]
async fn health_check_success() {
    spawn_app().await;

    let client = reqwest::Client::new();

    let response = client
        .get("http://localhost:8080/healthz")
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

async fn spawn_app() {
    zero2prod::run().await
}
