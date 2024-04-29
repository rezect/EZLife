use crate::HandlerResult;
use serde_json::json;
use reqwest::{Client, header};
use chrono::{Datelike, Local};
use std::fs::File;
use std::io::Read;

pub fn add_str_to_file(path: String, data: String, name_of_string: String) -> std::io::Result<()> {
    use std::fs::OpenOptions;
    use std::io::Write;

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(path)?;
    writeln!(file, "{}: {}", name_of_string, data)?;

    Ok(())
}

pub async fn add_new_to_notion(
    (energy, emotions, reflection, cur_date, chat_id): (String, String, String, String, String)
) -> HandlerResult {    
    let mut data_file = File::open(format!("user_conf/{}", chat_id)).expect("File not found");
    let mut database_id = String::new();
    data_file.read_to_string(&mut database_id).expect("File reading failed");
    database_id.pop();

    let url = "https://api.notion.com/v1/pages";
    let client = Client::new();
    let mut headers = header::HeaderMap::new();
    headers.insert("Authorization", header::HeaderValue::from_static("Bearer secret_XGRCGUudckdoUN6yO2eQRXeeQJ62IFNWaUkxexnAgFT"));
    headers.insert("Content-Type", header::HeaderValue::from_static("application/json"));
    headers.insert("Notion-Version", header::HeaderValue::from_static("2022-06-28"));

    // Получаем дату в формате 4 апр.
    let local_date = Local::now();
    let month_names = [
        "янв.", "фев", "марта", "апр.", "мая", "июня",
        "июля", "авг.", "сент.", "окт.", "ноября", "дек."
    ];
    let month_number = local_date.month() as usize;
    let month_name = month_names[month_number - 1];
    let day = local_date.day();

    // Формируем JSON-тело запроса
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
                            "content": "🧠Emotions:"
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

    let response = client
    .post(url.to_string())
    .headers(headers)
    .body(request_body.to_string())
    .send()
    .await?;

    if response.status().is_success() {
        // Получаем тело ответа как строку
        let body = response.text().await?;
        log::info!("Ответ сервера: {}", body);
    } else {
        log::error!("Ошибка: {:?}", response);
    }

    Ok(())
}

pub async fn add_new_reflection_to_notion(
    (reflection, chat_id): (String, String)
) -> HandlerResult {
    use chrono::prelude::*;

    let mut data_file = File::open(format!("user_conf/{}", chat_id)).expect("File not found");
    let mut database_id = String::new();
    data_file.read_to_string(&mut database_id).expect("File reading failed");

    let url = "https://api.notion.com/v1/pages";
    let client = Client::new();
    let mut headers = header::HeaderMap::new();
    headers.insert("Authorization", header::HeaderValue::from_static("Bearer secret_XGRCGUudckdoUN6yO2eQRXeeQJ62IFNWaUkxexnAgFT"));
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

    let response = client
    .post(url.to_string())
    .headers(headers)
    .body(request_body.to_string())
    .send()
    .await?;

    if response.status().is_success() {
        // Получаем тело ответа как строку
        let body = response.text().await?;
        log::info!("Ответ сервера: {}", body);
    } else {
        log::error!("Ошибка: {:?}", response);
    }

    Ok(())
}