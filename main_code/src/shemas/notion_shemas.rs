use serde_json::Value;

use crate::*;


pub async fn notion_shema_new_page(
    (energy, emotions, reflection, rate): (String, String, String, u32),
    chat_id: String,
) -> Response {

    // Получаем дату в формате "4 апр."
    let local_date: chrono::prelude::DateTime<Local>;
    if Local::now().hour() > 6 {
        local_date = Local::now();
    } else {
        local_date = Local::now() - chrono::Duration::days(1);
    }
    let month_names_for_tags = [
        "январь", "февраль", "март", "апрель", "май", "июнь",
        "июль", "август", "сентябрь", "октяюрь", "ноябрь", "декабрь"
    ];
    let cur_month_for_tags = month_names_for_tags[local_date.month0() as usize];
    let month_names = [
        "янв.", "фев.", "марта", "апр.", "мая", "июня",
        "июля", "авг.", "сент.", "окт.", "ноября", "дек."
    ];
    let month_name = month_names[local_date.month0() as usize];
    let day = local_date.day();
    let cur_date = Utc::now().format("%Y-%m-%d").to_string();

    let mut data_file = File::open(format!("user_db_ids/{}", chat_id)).expect("File not found");
    let mut database_id = String::new();
    data_file.read_to_string(&mut database_id).expect("File reading failed");
    database_id.pop();

    data_file = File::open(format!("user_tokens/{}", chat_id)).expect("File not found");
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
            "Tags": {
                "multi_select": [
                    {
                        "name": cur_month_for_tags
                    },
                    {
                        "name": "day"
                    },
                    {
                        "name": local_date.year().to_string()
                    },
                ]
            },
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
                "date": {
                    "start": cur_date
                }
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

    return json_data["access_token"].to_string();
}

pub async fn notion_db_test(
    chat_id: String,
    database_id: String
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
                            "content": "TEST"
                        }
                    }
                ]
            },
            "Date": {
                "date": {
                    "start": "1111-11-11"
                }
            },
            "Energy": {
                "select": {
                    "name": "Средняя энергия"
                }
            },
            "Rate": {
                "number": 0
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

pub async fn notion_edit_db(
    chat_id: String,
    db_id: String
) -> HandlerResult {
    let mut data_file = File::open(format!("user_tokens/{}", chat_id)).expect("File not found");
    let mut notion_token = String::new();
    data_file.read_to_string(&mut notion_token).expect("File reading failed");
    notion_token.pop();

    let url = format!("https://api.notion.com/v1/databases/{db_id}");
    let client = Client::new();
    let mut headers = header::HeaderMap::new();
    headers.insert("Authorization", header::HeaderValue::from_str(&format!("Bearer {}", notion_token)).expect("Invalid Notion token"));
    headers.insert("Content-Type", header::HeaderValue::from_static("application/json"));
    headers.insert("Notion-Version", header::HeaderValue::from_static("2022-06-28"));

    let response = client
        .get(url.to_string())
        .headers(headers.clone())
        .send()
        .await;

    let s = match response {
        Ok(response) => response.text().await.unwrap(),
        Err(_) => panic!("Error!"),
    };
    let json_data: serde_json::Value = serde_json::from_str(&s)
        .expect("Can't parse json");

    let tags_json_str = json_data["properties"]["Tags"].to_string();
    let tags_json: Value;
    if tags_json_str == "null" {
        tags_json = json!({
            "id": "flsb",
            "name": "Tags",
            "type": "multi_select",
            "multi_select": {}
        });
    } else {
        tags_json = serde_json::from_str(&tags_json_str).unwrap();
    }

    let request_body = json!({
        "properties": {
            "Tags": tags_json,
            "Date": {
                "id": "AJP%7D",
                "name": "Date",
                "type": "date",
                "date": {}
            },
            "Energy": {
                "id": "%40Q%5BM",
                "name": "Energy",
                "type": "select",
                "select": {
                  "options": [
                    {
                      "id": "e28f74fc-83a7-4469-8435-27eb18f9f9de",
                      "name": "Низкая энергия"
                    },
                    {
                      "id": "6132d771-b283-4cd9-ba44-b1ed30477c7f",
                      "name": "Средняя энергия"
                    },
                    {
                      "id": "fc9ea861-820b-4f2b-bc32-44ed9eca873c",
                      "name": "Высокая энергия"
                    }
                  ]
                }
            },
            "Rate": {
                "id": "%7B%5D_P",
                "name": "Rate",
                "type": "number",
                "number": {
                  "format": "number"
                }
            }
        }
    });

    let response1 = client
    .patch(url.to_string())
    .headers(headers)
    .body(request_body.to_string())
    .send()
    .await.expect("Failed to send request");

    println!("{}", response1.text().await.unwrap());

    Ok(())
}

pub async fn notion_reflection_shema(
    note_info: &str,
    chat_id: String,
) -> Response {

    let mut data_file = File::open(format!("user_db_ids/{}", chat_id)).expect("File not found");
    let mut database_id = String::new();
    data_file.read_to_string(&mut database_id).expect("File reading failed");
    database_id.pop();

    data_file = File::open(format!("user_tokens/{}", chat_id)).expect("File not found");
    let mut notion_token = String::new();
    data_file.read_to_string(&mut notion_token).expect("File reading failed");
    notion_token.pop();

    let local_time = Local::now();
    let cur_year = local_time.year();
    let cur_time = local_time.format("%H:%M").to_string();
    let month_names = [
        "января", "февраля", "марта", "апреля", "мая", "июня",
        "июля", "августа", "сентября", "октяюря", "ноября", "декабря"
    ];
    let month_names_for_tags = [
        "январь", "февраль", "март", "апрель", "май", "июнь",
        "июль", "август", "сентябрь", "октяюрь", "ноябрь", "декабрь"
    ];
    let cur_month = month_names[Local::now().month0() as usize];
    let cur_month_for_tags = month_names_for_tags[Local::now().month0() as usize];

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
            "Tags": {
                "multi_select": [
                    {
                        "name": cur_month_for_tags
                    },
                    {
                        "name": "note"
                    },
                    {
                        "name": cur_year.to_string()
                    },
                ]
            },
            "Name": {
                "title": [
                    {
                        "text": {
                            "content": local_time.day().to_string() + " " + cur_month
                        }
                    }
                ]
            },
            "Date": {
                "date": {
                    "start": Utc::now().format("%Y-%m-%dT%H:%MZ").to_string()
                }
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
                            "content": format!("Заметка в {cur_time}")
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
                                "content": note_info
                            }
                        }
                    ]
                }
            },
        ],
    });

    return client
        .post(url.to_string())
        .headers(headers.clone())
        .body(request_body.to_string())
        .send()
        .await.expect("Failed to send request");
}