use crate::*;


pub async fn notion_shema_new_page(
    (energy, emotions, reflection, rate, cur_date): (String, String, String, u32, String),
    (day, month_name, database_id): (u32, &str, String)

) -> Response {
    dotenv().ok();
    let notion_token = env::var("MY_NOTION_TOKEN").expect("MY_NOTION_TOKEN must be set in .env");


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

pub async fn notion_shema_add_reflection(
    reflection: String
) -> Response {
    dotenv().ok();
    let notion_token = env::var("MY_NOTION_TOKEN").expect("MY_NOTION_TOKEN must be set in .env");

    let url = "https://api.notion.com/v1/pages";
    let client = Client::new();
    let mut headers = header::HeaderMap::new();
    headers.insert("Authorization", header::HeaderValue::from_str(&format!("Bearer {}", notion_token)).expect("Invalid Notion token"));
    headers.insert("Content-Type", header::HeaderValue::from_static("application/json"));
    headers.insert("Notion-Version", header::HeaderValue::from_static("2022-06-28"));

    let request_body = json!({
        "children": [
            {
                "object": "block",
                "type": "heading_3",
                "heading_3": {
                    "rich_text": [
                        {
                            "type": "text",
                            "text": {
                            "content": format!("New note at {}:{}", Utc::now().hour(), Utc::now().minute())
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

