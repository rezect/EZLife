use crate::*;


// Функции-обработчики состояний
pub async fn start(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {

    tokio::time::sleep(Duration::from_millis(300)).await;
    bot.send_message(msg.chat.id, "Добро пожаловать, путник! Я бот, который будет выслушивать все твои жалобы и радости ;)").await?;
    tokio::time::sleep(Duration::from_millis(200)).await;
    bot.send_message(msg.chat.id, "Давай начнем с настройки Notion для более удобного хранения твоих записей?").await?;
    let chat_id = msg.chat.id.to_string();
    let user_name = msg.from().unwrap().username.to_owned().unwrap_or(String::from("NoName"));
    let path_str = format!("user_data/{}", chat_id);
    let path = Path::new(&path_str);
    if !path.exists() {
        let mut file = File::create(&path)?;
        writeln!(file, "Start documentation! Nickname - {}", user_name)?;
    }
    dialogue.update(State::ReceiveToNotion).await?;
    Ok(())
}

pub async fn receive_to_notion(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    dotenv().ok();
    let notion_acess_url = env::var("NOTION_ACESS_URL").expect("NOTION_ACESS_URL must be set in .env");


    match msg.text().unwrap_or("None").to_lowercase().as_str() {
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
            bot.send_message(msg.chat.id, "Тогда можешь позже попробовать, котик ;)").await?;
            tokio::time::sleep(Duration::from_millis(200)).await;
            bot.send_message(msg.chat.id, "Когда будешь готов поговорить про твой день, напиши мне /new").await?;
            dialogue.update(State::Waiting).await?;
        }
        _ => {
            tokio::time::sleep(Duration::from_millis(200)).await;
            bot.send_message(msg.chat.id, "Ладно, если захочешь подключить Notion, напиши мне /changedbid").await?;
            tokio::time::sleep(Duration::from_millis(200)).await;
            bot.send_message(msg.chat.id, "И когда будешь готов поговорить про твой день, напиши мне /new").await?;
            dialogue.update(State::Waiting).await?;
        }
    }
    Ok(())
}

pub async fn write_down_notion_token(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    let user_notion_token = get_notion_token_from_code(msg.text().unwrap_or("Get Notion token error").to_string()).await.trim_matches('"').to_string();
    if user_notion_token == "null" {
        tokio::time::sleep(Duration::from_millis(200)).await;
        bot.send_message(msg.chat.id, "Что-то не так с кодом который вы ввели, попробуйте еще раз.").await?;
        dialogue.update(State::GetNotionCode).await?;
    } else {
        let chat_id: String = msg.chat.id.to_string();
        let path_str = format!("user_tokens/{}", chat_id);
        let path = Path::new(&path_str);
        let mut file = File::create(&path)?;
        writeln!(file, "{}", user_notion_token)?;
        tokio::time::sleep(Duration::from_millis(200)).await;
        bot.send_message(msg.chat.id, "Все верной!\nЯ записал ваш токен.").await?;
        tokio::time::sleep(Duration::from_millis(200)).await;
        bot.send_message(msg.chat.id, "Теперь пришлите мне ссылку на базу данных, где вы планируете хранить ваши дни.").await?;
        dialogue.update(State::GetDBID).await?;
    }
    Ok(())
}

pub async fn get_db_id(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    match msg.text() {
        Some(url) => {
            let chat_id: String = msg.chat.id.to_string();
            let db_token = &url[22..(22 + 32)];
            let path_str = format!("user_db_ids/{}", chat_id);
            let path = Path::new(&path_str);
            let mut file = File::create(&path)?;
            writeln!(file, "{}", db_token)?;
            log::info!("Success to save notion token to file");
            tokio::time::sleep(Duration::from_millis(200)).await;
            bot.send_message(msg.chat.id, "Просто отлично!\nНастройка Notion завершена!").await?;
            tokio::time::sleep(Duration::from_millis(200)).await;
            bot.send_message(msg.chat.id, "Теперь когда будешь готов поговорить про твой день, напиши мне /new").await?;
            dialogue.update(State::Waiting).await?;
        }
        _ => {
            tokio::time::sleep(Duration::from_millis(200)).await;
            bot.send_message(msg.chat.id, "Отправь пж ссылОчку -_-").await?;
            dialogue.update(State::GetDBID).await?;
        }
    }
    Ok(())
}

pub async fn receive_energy(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
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

pub async fn receive_emotions(
    bot: Bot,
    dialogue: MyDialogue,
    energy: String,
    msg: Message,
) -> HandlerResult {
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

pub async fn receive_reflection(
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

pub async fn receive_rate(
    bot: Bot,
    dialogue: MyDialogue,
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

pub async fn is_all_ok(
    bot: Bot,
    dialogue: MyDialogue,
    (energy, emotions, reflection, rate): (String, String, String, u32),
    msg: Message
) -> HandlerResult {
    match msg.text().unwrap_or("None").to_lowercase().as_str() {
        "да" => {
            let date_time_string = Local::now().format("%d-%m-%Y %H:%M:%S").to_string();
            sleep(Duration::from_millis(200)).await;
            bot.send_message(msg.chat.id, "Хорошо, записал!\nДо встречи завтра ;)").await?;
            // Создаем страницу в Notion (пока только для меня)
            if msg.chat.id == ChatId(821961326) {
                match add_new_to_notion((energy.clone(), emotions.clone(), reflection.clone(), rate.clone(), date_time_string.clone(), msg.chat.id.to_string())).await {
                    Ok(_) => {
                        log::info!("Added to notion succsessfully");
                    }
                    Err(_) => {
                        log::warn!("Added to notion caused errors!");
                    }
                };
            }
            // Добавляем в локальную БД
            add_str_to_file(String::from(format!("user_data/{}", msg.chat.id.to_string())), date_time_string, String::from("Date"))?;
            add_str_to_file(String::from(format!("user_data/{}", msg.chat.id.to_string())), energy.clone(), String::from("Energy"))?;
            add_str_to_file(String::from(format!("user_data/{}", msg.chat.id.to_string())), emotions.clone(), String::from("Emotions"))?;
            add_str_to_file(String::from(format!("user_data/{}", msg.chat.id.to_string())), reflection.clone(), String::from("Reflection"))?;
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open(format!("user_data/{}", msg.chat.id.to_string()))?;
            writeln!(file, "")?;
            sleep(Duration::from_millis(300)).await;
            bot.send_message(msg.chat.id, "Соединяю с инопланетянами 👽 для анализа вашего дня...").await?;
            let smart_total = smart_total_result((energy, emotions, reflection)).await;
            sleep(Duration::from_millis(200)).await;
            bot.send_message(msg.chat.id, smart_total)
                .parse_mode(ParseMode::MarkdownV2)
                .await?;
            dialogue.update(State::Waiting).await?;
        }
        "нет" => {
            // В разработке
            sleep(Duration::from_millis(200)).await;
            bot.send_message(msg.chat.id, "Эта функция дорабатывается... Пока можете использовать /restart").await?;
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

pub async fn delete_handler(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message
) -> HandlerResult {
    sleep(Duration::from_millis(200)).await;
    match msg.text().unwrap_or("None").to_lowercase().as_str() {
        "да" => {
            let chat_id = msg.chat.id.to_string();
            let user_name = msg.from().unwrap().username.to_owned().unwrap_or(String::from("NoName"));
            let mut file = File::create(format!("user_data/{}", chat_id))?;
            writeln!(file, "Start documentation! Nickname - {}", user_name)?;
            bot.send_message(dialogue.chat_id(), "Ваши данные успешно удалены!").await?;
            dialogue.update(State::Waiting).await?;
        }
        "нет" => {
            bot.send_message(dialogue.chat_id(), "Ваши данные успешно удалены!").await?;
            sleep(Duration::from_secs(2)).await;
            bot.send_message(dialogue.chat_id(), "Ладно, шучу").await?;
            sleep(Duration::from_millis(200)).await;
            bot.send_message(dialogue.chat_id(), "Ваши данные в сохранности").await?;
            dialogue.update(State::Waiting).await?;
        }
        _ => {
            bot.send_message(msg.chat.id, "Я не понял твой ответ. Отправь либо Да либо Нет.").await?;
            dialogue.update(State::DeleteAllUserData).await?;
        }
    }
    Ok(())
}

pub async fn waiting_handler(
    bot: Bot,
    msg: Message,
    dialogue: MyDialogue,
) -> HandlerResult {
    match msg.text() {
        Some(text) => {
            sleep(Duration::from_millis(300)).await;
            bot.send_message(msg.chat.id, "Передаю инопланетянам 👽 ваш запрос...").await?;
            let smart_answer = smart_waiting_bot(text).await;
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
