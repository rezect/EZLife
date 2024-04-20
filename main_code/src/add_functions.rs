use teloxide::prelude::*;
use std::time::Duration;
use chrono::{Timelike, Local};
use crate::{State, MyDialogue, HandlerResult};

// Доп функции, не являющиеся обработчиками
pub async fn sleep_n_hours(n: u64) {
    tokio::time::sleep(std::time::Duration::from_secs(n * 3600)).await;
}

pub async fn sleep_next_day() {
    let since_midnight = {
        let now = Local::now().time();
        let seconds = now.num_seconds_from_midnight() as u64;
        seconds
    };
    let due: Duration;
    if since_midnight > 75600 {
        due = std::time::Duration::from_secs(86400 - since_midnight + 75600);
    } else {
        due = std::time::Duration::from_secs(75600 - since_midnight);
    }
    tokio::time::sleep(due).await;
}

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

// Функции для обработки ожидания пользователя (левая ветка в схеме)
pub async fn one_hour_ok(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    match msg.text() {
        Some("Да") => {
            bot.send_message(msg.chat.id, "Хорошо, напишу через часик").await?;
            sleep_n_hours(1).await;
            bot.send_message(msg.chat.id, "Час прошел, давай начнем с энергии").await?;
            bot.send_message(msg.chat.id, "Какая она была сегодня?").await?;
            dialogue.update(State::ReceiveEnergy).await?;
        }
        Some("Нет") => {
            bot.send_message(msg.chat.id, "Хорошо, а через два часа удобно будет поговорить?").await?;
            dialogue.update(State::TwoHourOk).await?;
        }
        _ => {
            bot.send_message(msg.chat.id, "Напиши только Да или Нет, зайка ;)").await?;
            dialogue.update(State::OneHourOk).await?;
        }
    }
    Ok(())
}

pub async fn two_hour_ok(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    match msg.text() {
        Some("Да") => {
            bot.send_message(msg.chat.id, "Хорошо, напишу через два часика").await?;
            sleep_n_hours(2).await;
            bot.send_message(msg.chat.id, "Время пришлоо, давай начнем с энергии").await?;
            bot.send_message(msg.chat.id, "Какая она была сегодня?").await?;
            dialogue.update(State::ReceiveEnergy).await?;
        }
        Some("Нет") => {
            sleep_next_day().await;
            bot.send_message(dialogue.chat_id(), "Привет, готов поговорить о прошедшем дне? ;)").await?;
            dialogue.update(State::ReceiveAgree).await?;
        }
        _ => {
            bot.send_message(msg.chat.id, "Напиши только Да или Нет, солнышко ;)").await?;
            dialogue.update(State::TwoHourOk).await?;
        }
    }
    Ok(())
}