use crate::*;
use std::fs::File;
use std::io::Write;
use chrono::Local;
use std::path::Path;
use std::time::Duration;
use crate::shemas::ya_gpt_shemas::*;


// Функции-обработчики состояний
pub async fn start(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {

    bot.send_message(msg.chat.id, "Добро пожаловать, путник! Я бот, который будет выслушивать все твои жалобы и радости ;)").await?;
    tokio::time::sleep(Duration::from_millis(100)).await;
    if msg.chat.id == ChatId(821961326) {
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
    } else {
        let smart_hello = smart_hello_asking().await;
        bot.send_message(msg.chat.id, smart_hello)
            .parse_mode(ParseMode::MarkdownV2)
            .await?;
        let chat_id = msg.chat.id.to_string();
        let user_name = msg.from().unwrap().username.to_owned().unwrap_or(String::from("NoName"));
        let path_str = format!("user_data/{}", chat_id);
        let path = Path::new(&path_str);
        if !path.exists() {
            let mut file = File::create(&path)?;
            writeln!(file, "Start documentation! Nickname - {}", user_name)?;
        }
        dialogue.update(State::ReceiveAgree).await?;
    }
    Ok(())
}

pub async fn receive_to_notion(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    match msg.text().unwrap_or("None").to_lowercase().as_str() {
        "да" => {
            bot.send_message(msg.chat.id, "Отлично, давай начнем!").await?;
            bot.send_message(msg.chat.id, "Мне от тебя нужна ссылка на страницу, где ты будешь хранить свои данные:").await?;
            dialogue.update(State::ReceiveNotionInfo).await?;
        }
        "нет" => {
            bot.send_message(msg.chat.id, "Тогда можешь позже попробовать, котик ;)").await?;
            let smart_hello = smart_hello_asking().await;
            bot.send_message(msg.chat.id, smart_hello)
                .parse_mode(ParseMode::MarkdownV2)
                .await?;
            dialogue.update(State::ReceiveAgree).await?;
        }
        _ => {}
    }
    Ok(())
}

pub async fn receive_notion_info(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    // https://www.notion.so/1ebb0c60b8864f668cc588eb9c816e91?v=8bc528af15334bda9803696341155178
    match msg.text() {
        Some(url) => {
            let chat_id: String = msg.chat.id.to_string();
            let db_token = &url[22..(22 + 32)];
            let path_str = format!("user_conf/{}", chat_id);
            let path = Path::new(&path_str);
            let mut file = File::create(&path)?;
            writeln!(file, "{}", db_token)?;
            log::info!("Success to save notion token to file");
            bot.send_message(msg.chat.id, "Спасибо, теперь я буду хранить твои данные на этой странице").await?;
            tokio::time::sleep(Duration::from_millis(100)).await;
            bot.send_message(msg.chat.id, "Теперь может поговорим про твой день?").await?;
            dialogue.update(State::ReceiveAgree).await?;
        }
        _ => {
            bot.send_message(msg.chat.id, "Отправь пж ссылОчку -_-").await?;
            dialogue.update(State::ReceiveNotionInfo).await?;
        }
    }
    Ok(())
}

pub async fn receive_agree(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    match msg.text().unwrap_or("None").to_lowercase().as_str() {
        "да" => {
            bot.send_message(msg.chat.id, "Хорошо, начнем с энергии").await?;
            bot.send_message(msg.chat.id, "Какая она была сегодня?").await?;
            dialogue.update(State::ReceiveEnergy).await?;
        }
        "нет" => {
            bot.send_message(msg.chat.id, "Тогда напиши когда удобно будет").await?;
            bot.send_message(msg.chat.id, "А я пока подремлю...").await?;
            dialogue.update(State::Waiting).await?;
        }
        _ => {
            bot.send_message(msg.chat.id, "Я не понял твой ответ. Отправь Да или Нет.").await?;
            dialogue.update(State::ReceiveAgree).await?;
        }
    }
    Ok(())
}

pub async fn receive_energy(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    match msg.text().unwrap_or("None").to_lowercase().as_str() {
        "низкая" => {
            bot.send_message(msg.chat.id, "Ничего страшного, это нормально бро").await?;
            bot.send_message(msg.chat.id, "Теперь расскажи о своих чувствах за сегодня").await?;
            dialogue.update(State::ReceiveEmotions { energy: String::from("Низкая энергия") }).await?;
        }
        "средняя" => {
            bot.send_message(msg.chat.id, "Главное во всем держать золотую середину ;)").await?;
            bot.send_message(msg.chat.id, "Теперь расскажи о своих чувствах за сегодня").await?;
            dialogue.update(State::ReceiveEmotions { energy: String::from("Средняя энергия") }).await?;
        }
        "высокая" => {
            bot.send_message(msg.chat.id, "Сегодня позитивненький день, получается :)").await?;
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
    tokio::time::sleep(Duration::from_millis(100)).await;
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
    use std::fs::OpenOptions;

    match msg.text().unwrap_or("None").to_lowercase().as_str() {
        "да" => {
            let date_time_string = Local::now().format("%d-%m-%Y %H:%M:%S").to_string();
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
            let smart_total = smart_total_result((energy, emotions, reflection)).await;
            bot.send_message(msg.chat.id, "Хорошо, записал!\nДо встречи завтра ;)").await?;
            bot.send_message(msg.chat.id, smart_total)
                .parse_mode(ParseMode::MarkdownV2)
                .await?;
            dialogue.update(State::Waiting).await?;
        }
        "нет" => {
            // В разработке
            bot.send_message(msg.chat.id, "Эта функция дорабатывается... Пока можете использовать /restart").await?;
            dialogue.update(State::IsAllOk { energy, emotions, reflection, rate }).await?;
        }
        _ => {
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
    use tokio::time::sleep;

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

pub async fn one_hour_ok_handler(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
) -> HandlerResult {
    match msg.text() {
        _ => {
            bot.send_message(msg.chat.id, "Эта функция еще не реализована :/").await?;
            dialogue.update(State::OneHourOk).await?;
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

pub async fn add_reflection_state_handler(bot: Bot, dialogue:MyDialogue, msg: Message) -> HandlerResult {
    match add_new_reflection_to_notion((String::from(msg.text().unwrap_or("ErrorReflection...")), msg.chat.id.to_string())).await {
        Ok(_) => {
            log::info!("Succsess to write reflection");
        }
        Err(err) => {
            log::error!("Error to write reflection: {}", err);
        }
    }
    bot.send_message(msg.chat.id, "Все записал, все зафиксировал. Приходи еще.").await?;
    dialogue.update(State::Waiting).await?;
    Ok(())
}
