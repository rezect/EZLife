use crate::*;


pub async fn notion_shema_new_page(
    (energy, emotions, reflection, rate, cur_date): (String, String, String, u32, String),
    (day, month_name, chat_id): (u32, &str, String)

) -> Response {
    let mut data_file = File::open(format!("user_db_ids/{}", chat_id)).expect("File not found 1");
    let mut database_id = String::new();
    data_file.read_to_string(&mut database_id).expect("File reading failed");
    database_id.pop();

    data_file = File::open(format!("user_tokens/{}", chat_id)).expect("File not found 2");
    let mut notion_token = String::new();
    data_file.read_to_string(&mut notion_token).expect("File reading failed");
    notion_token.pop();

    let url = "https://api.notion.com/v1/pages";
    let client = Client::new();
    let mut headers = header::HeaderMap::new();
    headers.insert("Authorization", header::HeaderValue::from_str(&format!("Bearer {}", notion_token)).expect("Invalid Notion token"));
    headers.insert("Content-Type", header::HeaderValue::from_static("application/json"));
    headers.insert("Notion-Version", header::HeaderValue::from_static("2022-06-28"));

    let request_body = json!({
        "parent": { "database_id": database_id },
        "icon": {
            "emoji": "🌇"
        },
        "properties": {
            "Name": {
                "title": [
                    {
                        "text": {
                            "content": day.to_string() + " " + month_name
                        }
                    }
                ]
            },
            "Date": {
                "rich_text": [
                    {
                        "text": {
                            "content": cur_date
                        }
                    }
                ]
            },
            "Energy": {
                "select": {
                    "name": energy
                }
            },
            "Rate": {
                "number": rate
            },
        },
        "children": [
            {
                "object": "block",
                "type": "heading_3",
                "heading_3": {
                    "rich_text": [
                        {
                            "type": "text",
                            "text": {
                            "content": "🎆Emotions:"
                            }
                        }
                    ]
                }
            },
            {
                "object": "block",
                "type": "paragraph",
                "paragraph": {
                    "rich_text": [
                        {
                            "type": "text",
                            "text": {
                                "content": emotions
                            }
                        }
                    ]
                }
            },
            {
                "object": "block",
                "type": "heading_3",
                "heading_3": {
                    "rich_text": [
                        {
                            "type": "text",
                            "text": {
                            "content": "🧠Reflection:"
                            }
                        }
                    ]
                }
            },
            {
                "object": "block",
                "type": "paragraph",
                "paragraph": {
                    "rich_text": [
                        {
                            "type": "text",
                            "text": {
                                "content": reflection
                            }
                        }
                    ]
                }
            },
        ],
    });

    return client
    .post(url.to_string())
    .headers(headers)
    .body(request_body.to_string())
    .send()
    .await.expect("Failed to send request");
}

pub async fn get_notion_token_from_code(
    code: String,
) -> String {
    dotenv().ok();
    let noton_basic_token = env::var("NOTION_BASE64_BASIC").expect("NOTION_BASE64_BASIC must be set in .env");
    let noton_redirect_url = env::var("NOTION_REDIRECT_URL").expect("NOTION_REDIRECT_URL must be set in .env");


    let url = "https://api.notion.com/v1/oauth/token";
    let client = Client::new();
    let mut headers = header::HeaderMap::new();
    headers.insert("Authorization", header::HeaderValue::from_str(format!("Basic {}", noton_basic_token).as_str()).expect("Invalid Notion tokens"));
    headers.insert("Content-Type", header::HeaderValue::from_static("application/json"));

    let request_body = json!({"grant_type":"authorization_code","code":code, "redirect_uri":noton_redirect_url});

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
    println!("{}", s);

    return json_data["access_token"].to_string();
}

pub async fn notion_db_test(
    chat_id: String,
    database_id: &str
) -> bool {
    let mut data_file = File::open(format!("user_tokens/{}", chat_id)).expect("File not found 2");
    let mut notion_token = String::new();
    data_file.read_to_string(&mut notion_token).expect("File reading failed");
    notion_token.pop();

    let url = "https://api.notion.com/v1/pages";
    let client = Client::new();
    let mut headers = header::HeaderMap::new();
    headers.insert("Authorization", header::HeaderValue::from_str(&format!("Bearer {}", notion_token)).expect("Invalid Notion token"));
    headers.insert("Content-Type", header::HeaderValue::from_static("application/json"));
    headers.insert("Notion-Version", header::HeaderValue::from_static("2022-06-28"));

    let request_body = json!({
        "parent": { "database_id": database_id },
        "icon": {
            "emoji": "🌇"
        },
        "properties": {
            "Name": {
                "title": [
                    {
                        "text": {
                            "content": "TEST PAGE"
                        }
                    }
                ]
            }
        },
        "children": [
            {
                "object": "block",
                "type": "heading_3",
                "heading_3": {
                    "rich_text": [
                        {
                            "type": "text",
                            "text": {
                            "content": "🎆Emotions:"
                            }
                        }
                    ]
                }
            },
            {
                "object": "block",
                "type": "paragraph",
                "paragraph": {
                    "rich_text": [
                        {
                            "type": "text",
                            "text": {
                                "content": "TEST"
                            }
                        }
                    ]
                }
            },
            {
                "object": "block",
                "type": "heading_3",
                "heading_3": {
                    "rich_text": [
                        {
                            "type": "text",
                            "text": {
                            "content": "🧠Reflection:"
                            }
                        }
                    ]
                }
            },
            {
                "object": "block",
                "type": "paragraph",
                "paragraph": {
                    "rich_text": [
                        {
                            "type": "text",
                            "text": {
                                "content": "TEST"
                            }
                        }
                    ]
                }
            },
        ],
    });

    let response = client
    .post(url.to_string())
    .headers(headers)
    .body(request_body.to_string())
    .send()
    .await.expect("Failed to send request");


    if response.status().is_success() {
        return true
    } else {
        println!("{}", response.text().await.unwrap());
        return false
    }
}