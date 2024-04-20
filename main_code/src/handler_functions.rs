use teloxide::prelude::*;
use crate::{State, MyDialogue, HandlerResult, sleep_next_day, add_str_to_file};
use std::fs::File;
use std::io::Write;
use chrono::Local;


// Функции-обработчики состояний
pub async fn start(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Привет, готов поговорить о прошедшем дне? ;)").await?;
    let chat_id = msg.chat.id.to_string();
    let user_name = msg.from().unwrap().username.to_owned().unwrap();
    let mut file = File::create(format!("user_data/{}", chat_id))?;
    writeln!(file, "Start documentation! Nickname - {}", user_name)?;
    dialogue.update(State::ReceiveAgree).await?;
    Ok(())
}

pub async fn receive_agree(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    match msg.text() {
        Some("Да") => {
            bot.send_message(msg.chat.id, "Хорошо, начнем с энергии").await?;
            bot.send_message(msg.chat.id, "Какая она была сегодня?").await?;
            dialogue.update(State::ReceiveEnergy).await?;
        }
        Some("Нет") => {
            bot.send_message(msg.chat.id, "Хорошо, а через час удобно будет поговорить?").await?;
            dialogue.update(State::OneHourOk).await?;
        }
        _ => {
            bot.send_message(msg.chat.id, "Напиши только Да или Нет, зайка ;)").await?;
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
            bot.send_message(msg.chat.id, "Сегодня позитивненький день, получается что-ли :)").await?;
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
            // let user_name = msg.from().unwrap().username.to_owned().unwrap();
            // bot.send_message(ChatId(821961326), format!("User: {}\nEnergy: {}\nEmotions: {}\nReflection: {}", user_name, energy, emotions, text)).await?;
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
            let dialogue_clone = dialogue.clone();
            let date_time_string = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
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
            bot.send_message(msg.chat.id, "Хорошо, записал!\nДо встречи завтра, хрючало ;)").await?;
            tokio::spawn( async move {
                sleep_next_day().await;
                bot.send_message(msg.chat.id, "Привет, готов поговорить о прошедшем дне? ;)").await.unwrap();
                dialogue.update(State::ReceiveAgree).await.unwrap();
            });
            dialogue_clone.update(State::Waiting).await?;
        }
        Some("Нет") => {
            // В разработке
            bot.send_message(msg.chat.id, "Эта функция дорабатывается...").await?;
        }
        _ => {
            bot.send_message(msg.chat.id, "Я не понял твой ответ. Отправь либо Да либо Нет.").await?;
            dialogue.update(State::IsAllOk { energy, emotions, reflection }).await?;
        }
    }
    Ok(())
}

pub async fn waiting_handler() -> HandlerResult {
    print!("Waiting handler...\n");
    Ok(())
}