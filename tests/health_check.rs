mod common;

#[tokio::test]
async fn health_check() {
    let (address, _) = common::spawn_app().await;

    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health_check", address))
        .send()
        .await
        .expect("falied to execute reqwest");

    //assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length())
}
