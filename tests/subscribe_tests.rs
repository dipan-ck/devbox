use crate::common::spawn_app;

mod common;

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data() {
    let (address, connection_pool) = spawn_app().await;

    let endpoint = format!("{}/subscriptions", address);
    let client = reqwest::Client::new();

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(endpoint)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&connection_pool)
        .await
        .expect("failed to fetch user data from Database");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn subscribe_returns_400_for_invalid_data() {
    let (address, _) = spawn_app().await;
    let data = [
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];
    let client = reqwest::Client::new();
    let endpoint = format!("{}/subscriptions", address);
    for (payload, _error_msg) in data {
        let response = client
            .post(&endpoint)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(payload)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(400, response.status().as_u16())
    }
}
