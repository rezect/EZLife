use serde_json::json;
use reqwest::{header, Client};


pub async fn yagpt_is_user_ready(
    user_answer: String,
) -> String {
    use dotenvy::dotenv;
    use std::env;


    dotenv().ok();
    let api_key = env::var("YAGPT_API_KEY").expect("YAGPT_API_KEY must be set in .env");
    let cloud_id = env::var("YAGPT_CLOUD_ID").expect("YAGPT_CLOUD_ID must be set in .env");
    let folder_id = env::var("YAGPT_FOLDER_ID").expect("YAGPT_FOLDER_ID must be set in .env");

    let url = "https://llm.api.cloud.yandex.net/foundationModels/v1/completion";
    let client = Client::new();
    let mut headers = header::HeaderMap::new();
    headers.insert("Authorization", header::HeaderValue::from_str(&format!("Api-Key {}", api_key)).expect("Invalid Api-Key token"));
    headers.insert("Content-Type", header::HeaderValue::from_static("application/json"));
    headers.insert("x-folder-id", header::HeaderValue::from_str(&cloud_id).expect("Invalid folder id"));

    let request_body = json!(
        {
            "modelUri": format!("gpt://{}/yandexgpt/latest", folder_id),
            "completionOptions": {
                  "stream": true,
                  "temperature": 0.3,
                  "maxTokens": 500
              },
              "messages": [
                {
                    "role": "system",
                    "text": "На вход подается ответ пользователя на вопрос 'Готов поговорить о прошедшем дне?'. Ты должен: вывести Да, если пользователь готов, вывести Нет если он не готов или готов в другое время, вывести NS если из ответа невозможно понять готов пользователь или нет. Ответ должен быть только одним словом - (Да, либо Нет, либо NS)."
                },
                {
                    "role": "user",
                    "text": user_answer
                },
            ]
        }
    );
    

    let response = client
    .post(url.to_string())
    .headers(headers)
    .body(request_body.to_string())
    .send()
    .await.expect("Failed to send request");

    // print!("{:?}", response.text().await);

    return response.text().await.unwrap_or("Error".to_string());
}

