use crate::*;


pub async fn smart_waiting_bot(
    user_answer: &str,
) -> String {
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
                  "stream": false,
                  "temperature": 0.3,
                  "maxTokens": 500
              },
              "messages": [
                {
                    "role": "system",
                    "text": "Ты - умный помошник и друг. Твоя задача - выслушивать пользователя и немного помогать советами."
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
        .await;
    let s = match response {
        Ok(response) => response.text().await.unwrap(),
        Err(_) => panic!("Error!"),
    };
    let json_data: serde_json::Value = serde_json::from_str(&s)
        .expect("Can't parse json");

    let mut text_response = json_data["result"]["alternatives"][0]["message"]["text"].to_string()[1..].to_string();
    text_response.pop().unwrap();

    return format_str(&text_response);
}

pub async fn smart_total_result(
    (energy, emotions, reflection): (String, String, String)
) -> String {
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
    let total_text = format!("Энергия за день была: {}\nЭмоции за день были: {}\nОтзыв за день: {}", energy, emotions, reflection);

    let request_body = json!(
        {
            "modelUri": format!("gpt://{}/yandexgpt/latest", folder_id),
            "completionOptions": {
                  "stream": false,
                  "temperature": 0.5,
                  "maxTokens": 100
              },
              "messages": [
                {
                    "role": "system",
                    "text": "Твоя задача - похвалить человека в двух предложениях на основе данных о его прошедшем дне."
                },
                {
                    "role": "user",
                    "text": total_text
                },
            ]
        }
    );

    let response = client
        .post(url.to_string())
        .headers(headers)
        .body(request_body.to_string())
        .send()
        .await;

    let s = match response {
        Ok(response) => response.text().await.unwrap(),
        Err(_) => panic!("Error!"),
    };

    let json_data: serde_json::Value = serde_json::from_str(&s)
        .expect("Can't parse json");

    let mut text_response = json_data["result"]["alternatives"][0]["message"]["text"].to_string()[1..].to_string();
    text_response.pop().unwrap();

    return format_str(&text_response);
}