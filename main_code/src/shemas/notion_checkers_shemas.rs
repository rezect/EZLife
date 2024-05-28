use crate::*;


pub async fn notion_is_token_valid(
    token_to_check: String
) -> bool {
    
    let url = format!("https://api.notion.com/v1/users/me");
    let client = Client::new();
    let mut headers = header::HeaderMap::new();
    headers.insert("Authorization", header::HeaderValue::from_str(&format!("Bearer {}", token_to_check)).expect("Invalid Notion token"));
    headers.insert("Content-Type", header::HeaderValue::from_static("application/json"));
    headers.insert("Notion-Version", header::HeaderValue::from_static("2022-06-28"));

    let response = client
        .get(url.to_string())
        .headers(headers)
        .send()
        .await
        .expect("Failed to send request");

    return response.status().is_success();
}

pub async fn notion_db_check(
    database_to_check: String,
    notion_token: String,
) -> bool {
    
    let url = format!("https://api.notion.com/v1/databases/{database_to_check}");
    let client = Client::new();
    let mut headers = header::HeaderMap::new();
    headers.insert("Authorization", header::HeaderValue::from_str(&format!("Bearer {}", notion_token)).expect("Invalid Notion token"));
    headers.insert("Content-Type", header::HeaderValue::from_static("application/json"));
    headers.insert("Notion-Version", header::HeaderValue::from_static("2022-06-28"));

    let response = client
        .get(url.to_string())
        .headers(headers.clone())
        .send()
        .await.expect("Cant send request");

    return response.status().is_success();
}