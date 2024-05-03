use crate::*;
use std::fs::File;
use std::io::Write;
use chrono::Local;
// use tokio_cron_scheduler::{Job, JobScheduler};
use std::path::Path;

// Функции-обработчики состояний
pub async fn start(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {

    // Удаленное напоминание каждый день в 9 вечера по МСК
    /*let sched = JobScheduler::new().await?;
    let bot_clone = bot.clone();
    let msg_clone = msg.clone();
    let dialogue_clone = dialogue.clone();
    sched.add(
        Job::new_async("00 00 18 * * ? *", move |_uuid, _lock| {
            let bot_clone = bot_clone.clone();
            let msg_clone = msg_clone.clone();
            let dialogue_clone = dialogue_clone.clone();
            Box::pin( async move {
                match bot_clone.send_message(msg_clone.chat.id, "Ку, готов поговорить про твой день?").await {
                    Ok(_) => {
                        log::info!("Success to send message 'Ку, готов поговорить про твой день?'", );
                    }
                    Err(err) => {
                        log::warn!("Failed to send message: {}", err);
                    }
                }
                match dialogue_clone.update(State::ReceiveAgree).await {
                    Ok(_) => {
                        log::info!("Success update to ReceiveAgree");
                    }
                    Err(err) => {
                        log::warn!("Failed to update dialogue state: {}", err);
                    }
                }
            })
        })?
    ).await?;
    sched.start().await?;*/
    use std::time::Duration;

    bot.send_message(msg.chat.id, "Добро пожаловать, путник! Я бот, который будет выслушивать все твои жалобы и радости ;)").await?;
    tokio::time::sleep(Duration::from_secs(2)).await;
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
        bot.send_message(msg.chat.id, "Как прошел твой день? Готов поговорить про него?").await?;
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
    match msg.text() {
        Some("Да") => {
            bot.send_message(msg.chat.id, "Отлично, давай начнем!").await?;
            bot.send_message(msg.chat.id, "Мне от тебя нужна ссылка на страницу, где ты будешь хранить свои данные:").await?;
            dialogue.update(State::ReceiveNotionInfo).await?;
        }
        Some("Нет") => {
            bot.send_message(msg.chat.id, "Тогда можешь позже попробовать, котик ;)").await?;
            bot.send_message(msg.chat.id, "Поговорим про твой день?").await?;
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
    let ya_response = is_user_ready_ai(msg.text().unwrap_or("NoText").to_owned()).await;
    match ya_response {
        'Д' => {
            bot.send_message(msg.chat.id, "Хорошо, начнем с энергии").await?;
            bot.send_message(msg.chat.id, "Какая она была сегодня?").await?;
            dialogue.update(State::ReceiveEnergy).await?;
        }
        'Н' => {
            bot.send_message(msg.chat.id, "Тогда напиши Да когда удобно будет").await?;
            dialogue.update(State::Waiting).await?;
        }
        'N' => {
            bot.send_message(msg.chat.id, "Неочень тебя понял, напиши еще раз").await?;
            dialogue.update(State::Waiting).await?;
        }
        _ => {
            bot.send_message(msg.chat.id, "Я не понял твой ответ. Попробуй еще раз.").await?;
            dialogue.update(State::ReceiveAgree).await?;
        }
    }
    Ok(())
}

pub async fn receive_energy(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    match msg.text() {
        Some("Низкая") => {
            bot.send_message(msg.chat.id, "Ничего страшного, это нормально бро").await?;
            bot.send_message(msg.chat.id, "Теперь расскажи о своих чувствах за сегодня").await?;
            dialogue.update(State::ReceiveEmotions { energy: String::from("Низкая энергия") }).await?;
        }
        Some("Средняя") => {
            bot.send_message(msg.chat.id, "Главное во всем держать золотую середину ;)").await?;
            bot.send_message(msg.chat.id, "Теперь расскажи о своих чувствах за сегодня").await?;
            dialogue.update(State::ReceiveEmotions { energy: String::from("Средняя энергия") }).await?;
        }
        Some("Высокая") => {
            bot.send_message(msg.chat.id, "Сегодня позитивненький день, получается :)").await?;
            bot.send_message(msg.chat.id, "Теперь расскажи о своих чувствах за сегодня").await?;
            dialogue.update(State::ReceiveEmotions { energy: String::from("Высокая энергия") }).await?;
        }
        _ => {
            bot.send_message(msg.chat.id, "Напиши одну из трех категорий: 'Низкая', 'Средняя', 'Высокая'").await?;
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
    match msg.text() {
        Some(text) => {
            bot.send_message(msg.chat.id, format!("Отлично, день закончен, поздравляю!\nВот краткий итог:")).await?;
            bot.send_message(msg.chat.id, format!("Energy: {}\nEmotions: {}\nReflection: {}", energy, emotions, text)).await?;
            bot.send_message(msg.chat.id, "Всё верно?").await?;
            dialogue.update(State::IsAllOk { energy, emotions, reflection: (text.into()) }).await?;
        }
        _ => {
            bot.send_message(msg.chat.id, "Напиши что-нибудь, котик ;)").await?;
            dialogue.update(State::ReceiveReflection { energy, emotions }).await?;
        }
    }
    Ok(())
}

pub async fn is_all_ok(
    bot: Bot,
    dialogue: MyDialogue,
    (energy, emotions, reflection): (String, String, String),
    msg: Message
) -> HandlerResult {
    use std::fs::OpenOptions;

    match msg.text() {
        Some("Да") => {
            let date_time_string = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            // Создаем страницу в Notion
            if msg.chat.id == ChatId(821961326) {
                match add_new_to_notion((energy.clone(), emotions.clone(), reflection.clone(), date_time_string.clone(), msg.chat.id.to_string())).await {
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
            add_str_to_file(String::from(format!("user_data/{}", msg.chat.id.to_string())), energy, String::from("Energy"))?;
            add_str_to_file(String::from(format!("user_data/{}", msg.chat.id.to_string())), emotions, String::from("Emotions"))?;
            add_str_to_file(String::from(format!("user_data/{}", msg.chat.id.to_string())), reflection, String::from("Reflection"))?;
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open(format!("user_data/{}", msg.chat.id.to_string()))?;
            writeln!(file, "")?;
            bot.send_message(msg.chat.id, "Хорошо, записал!\nДо встречи завтра ;)").await?;
            dialogue.update(State::Waiting).await?;
        }
        Some("Нет") => {
            // В разработке
            bot.send_message(msg.chat.id, "Эта функция дорабатывается... Пока можете использовать /restart").await?;
            dialogue.update(State::IsAllOk { energy, emotions, reflection }).await?;
        }
        _ => {
            bot.send_message(msg.chat.id, "Я не понял твой ответ. Отправь либо Да либо Нет.").await?;
            dialogue.update(State::IsAllOk { energy, emotions, reflection }).await?;
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
    use std::time::Duration;
    match msg.text() {
        Some("Да") => {
            let chat_id = msg.chat.id.to_string();
            let user_name = msg.from().unwrap().username.to_owned().unwrap_or(String::from("NoName"));
            let mut file = File::create(format!("user_data/{}", chat_id))?;
            writeln!(file, "Start documentation! Nickname - {}", user_name)?;
            bot.send_message(dialogue.chat_id(), "Ваши данные успешно удалены!").await?;
            dialogue.update(State::Waiting).await?;
        }
        Some("Нет") => {
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
            bot.send_message(dialogue.chat_id(), smart_answer).await?;
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
