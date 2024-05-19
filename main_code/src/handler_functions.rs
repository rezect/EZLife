use crate::*;


// Функции-обработчики состояний
pub async fn start_handler(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {

    tokio::time::sleep(Duration::from_millis(300)).await;
    bot.send_message(msg.chat.id, "Добро пожаловать, путник!🎒\nЯ бот, который будет выслушивать все твои жалобы и радости ;)").await?;
    tokio::time::sleep(Duration::from_millis(200)).await;
    bot.send_message(msg.chat.id, "Давай начнем с настройки Notion для более удобного хранения твоих записей?").await?;

    dialogue.update(State::ReceiveToNotion).await?;
    Ok(())
}

pub async fn receive_to_notion_handler(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {

    dotenv().ok();
    let notion_acess_url = env::var("NOTION_ACESS_URL").expect("NOTION_ACESS_URL must be set in .env");

    match msg.text().unwrap_or_default().to_lowercase().as_str() {
        "да" => {
            tokio::time::sleep(Duration::from_millis(200)).await;
            bot.send_message(msg.chat.id, "Отлично, давай начнем!").await?;
            tokio::time::sleep(Duration::from_millis(200)).await;
            let ask_to_url = format!("Мне от тебя нужен токен, который ты получишь по ссылке: [*тык*]({})", notion_acess_url);
            bot.send_message(msg.chat.id, ask_to_url)
                .parse_mode(ParseMode::MarkdownV2)
                .await?;
            dialogue.update(State::GetNotionCode).await?;
        }
        "нет" => {
            tokio::time::sleep(Duration::from_millis(200)).await;
            bot.send_message(msg.chat.id, "Тогда можешь позже попробовать, котик - /notion ;)\nНо большинство функций будет недоступно, потому что мы не храним данные локально :/").await?;
            tokio::time::sleep(Duration::from_millis(200)).await;
            bot.send_message(msg.chat.id, "Когда будешь готов поговорить про твой день, напиши мне /day").await?;
            dialogue.update(State::Waiting).await?;
        }
        _ => {
            tokio::time::sleep(Duration::from_millis(200)).await;
            bot.send_message(msg.chat.id, "Ладно, если захочешь подключить Notion, напиши мне /notion").await?;
            tokio::time::sleep(Duration::from_millis(200)).await;
            bot.send_message(msg.chat.id, "А пока можешь задавать мне вопросы, я тебя выслушаю и помогу советом.").await?;
            dialogue.update(State::Waiting).await?;
        }
    }

    Ok(())
}

pub async fn get_notion_code_handler(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {

    let user_notion_token: String = get_notion_token_from_code(msg.text().unwrap_or("Get Notion token error").to_string()).await.trim_matches('"').to_string();
    if notion_is_token_valid(user_notion_token.clone()).await {
        let chat_id: String = msg.chat.id.to_string();

        if !Path::new("user_tokens").exists() {
            match std::fs::create_dir("user_tokens") {
                Ok(_) => println!("Директория 'user_tokens' успешно создана."),
                Err(e) => eprintln!("Ошибка при создании директории user_tokens: {}", e),
            }
        }

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(format!("user_tokens/{}", chat_id))?;
        writeln!(file, "{}", user_notion_token)?;

        tokio::time::sleep(Duration::from_millis(200)).await;
        bot.send_message(msg.chat.id, "Все верно!\nЯ записал ваш токен.").await?;
        tokio::time::sleep(Duration::from_millis(200)).await;
        bot.send_message(msg.chat.id, "Теперь пришлите мне ссылку на базу данных, где вы планируете хранить ваши дни.").await?;
        tokio::time::sleep(Duration::from_millis(200)).await;
        bot.send_message(msg.chat.id, "Как создать базу данных Notion \\- посмотрите [*тут*](https://www.notion.so/help/create-a-database)\\.")
            .parse_mode(ParseMode::MarkdownV2)
            .await?;
        dialogue.update(State::GetDBID).await?;
    } else {
        tokio::time::sleep(Duration::from_millis(200)).await;
        bot.send_message(msg.chat.id, "Что-то не так с кодом который вы ввели, попробуйте еще раз.").await?;
        dialogue.update(State::GetNotionCode).await?;
    }

    Ok(())
}

pub async fn get_db_id_handler(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {

    match msg.text() {
        Some(url) => {

            let db_token: &str;
            if url.chars().count() >= 54 {
                db_token = &url[22..(22 + 32)];
            } else {
                db_token = "Invalid link";
            }

            notion_edit_db(msg.chat.id.to_string(), db_token).await?;
            sleep(Duration::from_millis(50)).await;
            let response_is_success = notion_db_test(msg.chat.id.to_string(), db_token).await;

            if response_is_success {

                if !Path::new("user_db_ids").exists() {
                    match std::fs::create_dir("user_db_ids") {
                        Ok(_) => println!("Директория 'user_db_ids' успешно создана."),
                        Err(e) => eprintln!("Ошибка при создании директории user_db_ids: {}", e),
                    }
                }

                let mut file = OpenOptions::new()
                    .write(true)
                    .append(false)
                    .create(true)
                    .open(format!("user_db_ids/{}", msg.chat.id))?;
                writeln!(file, "{}", db_token)?;
                log::info!("Success to save notion token to file");
                
                tokio::time::sleep(Duration::from_millis(200)).await;
                bot.send_message(msg.chat.id, "Просто отлично!\nНастройка Notion завершена!").await?;
                tokio::time::sleep(Duration::from_millis(200)).await;
                bot.send_message(msg.chat.id, "В вашей базе данных была создана 'тестовая' страница.").await?;
                tokio::time::sleep(Duration::from_millis(200)).await;
                bot.send_message(msg.chat.id, "Теперь когда будешь готов поговорить про твой день, напиши мне /day").await?;
                dialogue.update(State::Waiting).await?;
            } else {
                tokio::time::sleep(Duration::from_millis(200)).await;
                bot.send_message(msg.chat.id, "Неправильная ссылка на базу данных, попробуй еще раз. :(").await?;
                tokio::time::sleep(Duration::from_millis(200)).await;
                let mut photo = PathBuf::new();
                photo.push("images/guide_db_link.png");
                bot.send_photo(msg.chat.id, InputFile::file(photo)).await?;
                bot.send_message(msg.chat.id, "Перейдите на страницу своей базы данных и нажмите на три точки справа сверху.\nДалее скопируйте ссылку (Copy link) и отправьте ее мне.").await?;
                tokio::time::sleep(Duration::from_millis(200)).await;
                bot.send_message(msg.chat.id, "Если не получается - напишите моему хозяину: @rezect 🧑‍💻").await?;
                dialogue.update(State::GetDBID).await?;
            }
        }
        _ => {
            tokio::time::sleep(Duration::from_millis(200)).await;
            bot.send_message(msg.chat.id, "Отправь пж ссылОчку -_-").await?;
        }
    }

    Ok(())
}

pub async fn receive_energy_handler(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {

    tokio::time::sleep(Duration::from_millis(200)).await;
    match msg.text().unwrap_or("None").to_lowercase().as_str() {
        "низкая" => {
            bot.send_message(msg.chat.id, "Ничего страшного, это нормально бро").await?;
            tokio::time::sleep(Duration::from_millis(200)).await;
            bot.send_message(msg.chat.id, "Теперь расскажи о своих чувствах за сегодня").await?;
            dialogue.update(State::ReceiveEmotions { energy: String::from("Низкая энергия") }).await?;
        }
        "средняя" => {
            bot.send_message(msg.chat.id, "Главное во всем держать золотую середину ;)").await?;
            tokio::time::sleep(Duration::from_millis(200)).await;
            bot.send_message(msg.chat.id, "Теперь расскажи о своих чувствах за сегодня").await?;
            dialogue.update(State::ReceiveEmotions { energy: String::from("Средняя энергия") }).await?;
        }
        "высокая" => {
            bot.send_message(msg.chat.id, "Сегодня позитивненький день, получается :)").await?;
            tokio::time::sleep(Duration::from_millis(200)).await;
            bot.send_message(msg.chat.id, "Теперь расскажи о своих чувствах за сегодня").await?;
            dialogue.update(State::ReceiveEmotions { energy: String::from("Высокая энергия") }).await?;
        }
        _ => {
            bot.send_message(msg.chat.id, "Напиши одну из трех категорий: 'Низкая', 'Средняя' или 'Высокая'").await?;
            dialogue.update(State::ReceiveEnergy).await?;
        }
    }

    Ok(())
}

pub async fn receive_emotions_handler(bot: Bot, dialogue: MyDialogue, energy: String, msg: Message) -> HandlerResult {

    tokio::time::sleep(Duration::from_millis(200)).await;
    match msg.text() {
        Some(text) => {
            bot.send_message(msg.chat.id, "Теперь можешь поделиться впечатлениями о дне").await?;
            dialogue.update(State::ReceiveReflection { energy, emotions: (String::from(text)) }).await?;
        }
        _ => {
            bot.send_message(msg.chat.id, "Напиши что-нибудь, котик ;)").await?;
            dialogue.update(State::ReceiveEmotions { energy }).await?;
        }
    }

    Ok(())
}

pub async fn receive_reflection_handler(
    bot: Bot,
    dialogue: MyDialogue,
    (energy, emotions): (String, String),
    msg: Message
) -> HandlerResult {
    tokio::time::sleep(Duration::from_millis(200)).await;
    match msg.text() {
        Some(text) => {
            bot.send_message(msg.chat.id, format!("Какую дашь общую оценку дню? (от 0 до 10)")).await?;
            dialogue.update(State::ReceiveRate { energy, emotions, reflection: (text.into()) }).await?;
        }
        _ => {
            bot.send_message(msg.chat.id, "Напиши что-нибудь, котик ;)").await?;
            dialogue.update(State::ReceiveReflection { energy, emotions }).await?;
        }
    }
    Ok(())
}

pub async fn receive_rate_handler(
    bot: Bot, dialogue: MyDialogue,
    (energy, emotions, reflection): (String, String, String),
    msg: Message
) -> HandlerResult {

    tokio::time::sleep(Duration::from_millis(200)).await;
    match msg.text().unwrap_or("Error").to_owned().parse::<u32>() {
        Ok(num) => {
            if num > 10 {
                bot.send_message(msg.chat.id, "Отправь число не больше 10").await?;
                dialogue.update(State::ReceiveRate { energy, emotions, reflection }).await?;
                return Ok(())
            }
            bot.send_message(msg.chat.id, format!("Отлично, день закончен, поздравляю!\nВот краткий итог:")).await?;
            tokio::time::sleep(Duration::from_millis(200)).await;
            bot.send_message(msg.chat.id, format!("Energy: {}\nEmotions: {}\nRate: {}", energy, emotions, num)).await?;
            tokio::time::sleep(Duration::from_millis(200)).await;
            bot.send_message(msg.chat.id, "Всё верно?").await?;
            dialogue.update(State::IsAllOk { energy, emotions, reflection, rate: (num) }).await?;
        }
        Err(_) => {
            bot.send_message(msg.chat.id, "Отправь число от 0 до 10").await?;
            dialogue.update(State::ReceiveRate { energy, emotions, reflection }).await?;
        }
    }

    Ok(())
}

pub async fn is_all_ok_handler(
    bot: Bot, dialogue: MyDialogue,
    (energy, emotions, reflection, rate): (String, String, String, u32),
    msg: Message
) -> HandlerResult {

    match msg.text().unwrap_or("None").to_lowercase().as_str() {
        "да" => {
            let date_time_string = Local::now().format("%d.%m.%Y %H:%M").to_string();
            sleep(Duration::from_millis(200)).await;
            bot.send_message(msg.chat.id, "Хорошо, записал!\nДо встречи завтра ;)").await?;

            match add_new_to_notion((energy.clone(), emotions.clone(), reflection.clone(), rate.clone(), date_time_string.clone(), msg.chat.id.to_string(), bot.clone())).await {
                Ok(_) => {
                    log::info!("Added to notion succsessfully");
                }
                Err(_) => {
                    log::warn!("Added to notion caused errors!");
                }
            };

            sleep(Duration::from_millis(300)).await;
            bot.send_message(msg.chat.id, "Соединяю с инопланетянами 👽 для анализа вашего дня...").await?;

            let mut smart_total = smart_total_result((energy.clone(), emotions.clone(), reflection.clone())).await;
            if smart_total == "ul" {
                bot.send_message(msg.chat.id, "Сейчас я немного занят🤯, попытаюсь еще раз через 5 сек").await?;
                sleep(Duration::from_secs(5)).await;
                smart_total = smart_total_result((energy.clone(), emotions.clone(), reflection.clone())).await;
            }
            while smart_total == "ul" {
                bot.send_message(msg.chat.id, "Иииии еще раз😓...").await?;
                sleep(Duration::from_secs(5)).await;
                smart_total = smart_total_result((energy.clone(), emotions.clone(), reflection.clone())).await;
            }

            sleep(Duration::from_millis(200)).await;
            bot.send_message(msg.chat.id, smart_total)
                .parse_mode(ParseMode::MarkdownV2)
                .await?;
            dialogue.update(State::Waiting).await?;
        }
        "нет" => {
            // В разработке
            sleep(Duration::from_millis(200)).await;
            bot.send_message(msg.chat.id, "Эта функция дорабатывается...\nПока можете использовать /day чтобы заново описать свой день").await?;
            dialogue.update(State::Waiting).await?;
        }
        _ => {
            sleep(Duration::from_millis(200)).await;
            bot.send_message(msg.chat.id, "Я не понял твой ответ. Отправь либо Да либо Нет.").await?;
            dialogue.update(State::IsAllOk { energy, emotions, reflection, rate }).await?;
        }
    }

    Ok(())
}

pub async fn waiting_handler(bot: Bot, msg: Message, dialogue: MyDialogue) -> HandlerResult {

    match msg.text() {
        Some(text) => {
            sleep(Duration::from_millis(300)).await;
            bot.send_message(msg.chat.id, "Передаю ваш запрос инопланетянам 👽...").await?;

            let mut smart_answer = smart_waiting_bot(text).await;
            if smart_answer == "ul" {
                bot.send_message(msg.chat.id, "Сейчас я немного занят🤯, попытаюсь еще раз через 5 секунд").await?;
                sleep(Duration::from_secs(5)).await;
                smart_answer = smart_waiting_bot(text).await;
            }
            while smart_answer == "ul" {
                bot.send_message(msg.chat.id, "Иииии еще раз😓...").await?;
                sleep(Duration::from_secs(3)).await;
                smart_answer = smart_waiting_bot(text).await;
            }

            bot.send_message(msg.chat.id, smart_answer)
                .parse_mode(ParseMode::MarkdownV2)
                .await?;
        }
        _ => {
            bot.send_message(dialogue.chat_id(), "Я не понял твой ответ. Отправь мне что-нибудь... текстовое").await?;
        }
    }

    Ok(())
}
