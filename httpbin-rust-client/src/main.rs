use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize)]
struct PostRequest {
    key: String,
    value: String,
}

#[derive(Deserialize)]
struct PostResponse {
    result: String,
}

#[derive(Deserialize)]
struct GetResponse {
    key: String,
    value: String,
}

#[tokio::main]
async fn main() {
    let base_url = "http://127.0.0.1:3000";
    let client = Client::new();

    if let Err(err) = test_get(&client, base_url).await {
        eprintln!("GET test failed: {}", err);
    }

    if let Err(err) = test_post(&client, base_url).await {
        eprintln!("POST test failed: {}", err);
    }
}

async fn test_get(client: &Client, base_url: &str) -> Result<(), Box<dyn Error>> {
    let key = "test";
    let response: GetResponse = client
        .get(&format!("{}/get/{}", base_url, key))
        .send()
        .await?
        .json()
        .await?;

    assert_eq!(response.key, key);
    assert_eq!(response.value, "Hello, Axum!");

    println!("GET test passed!");

    Ok(())
}

async fn test_post(client: &Client, base_url: &str) -> Result<(), Box<dyn Error>> {
    let request = PostRequest {
        key: "test_key".to_string(),
        value: "test_value".to_string(),
    };

    let response: PostResponse = client
        .post(&format!("{}/post", base_url))
        .json(&request)
        .send()
        .await?
        .json()
        .await?;

    let expected_result = "Received key: test_key, value: test_value";
    assert_eq!(response.result, expected_result);

    println!("POST test passed!");

    Ok(())
}
