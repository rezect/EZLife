use teloxide::prelude::*;
use crate::{State, MyDialogue, HandlerResult, sleep_next_day};

// Функции-обработчики состояний
pub async fn start(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Привет, готов поговорить о прошедшем дне? ;)").await?;
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
            // Реализация записи energy в файл
            dialogue.update(State::ReceiveEmotions { energy: String::from("Низкая энергия") }).await?;
        }
        Some("Средняя") => {
            bot.send_message(msg.chat.id, "Главное во всем держать золотую середину ;)").await?;
            bot.send_message(msg.chat.id, "Теперь расскажи о своих чувствах за сегодня").await?;
            // Реализация записи energy в файл
            dialogue.update(State::ReceiveEmotions { energy: String::from("Средняя энергия") }).await?;
        }
        Some("Высокая") => {
            bot.send_message(msg.chat.id, "Сегодня позитивненький день, получается что-ли :)").await?;
            bot.send_message(msg.chat.id, "Теперь расскажи о своих чувствах за сегодня").await?;
            // Реализация записи energy в файл
            dialogue.update(State::ReceiveEmotions { energy: String::from("Высокая энергия") }).await?;
        }
        _ => {
            bot.send_message(msg.chat.id, "Напиши одну из трех категорий: низкая, средняя, высокая").await?;
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
            // Реализация записи emotions в файл
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
            let user_name = msg.from().unwrap().username.to_owned().unwrap();
            bot.send_message(ChatId(821961326), format!("User: {}\nEnergy: {}\nEmotions: {}\nReflection: {}", user_name, energy, emotions, text)).await?;
            // Реализация записи reflection в файл
            sleep_next_day().await;
            bot.send_message(dialogue.chat_id(), "Привет, готов поговорить о прошедшем дне? ;)").await?;
            dialogue.update(State::ReceiveAgree).await?;
        }
        _ => {
            bot.send_message(msg.chat.id, "Напиши что-нибудь, котик ;)").await?;
            dialogue.update(State::ReceiveReflection { energy, emotions }).await?;
        }
    }
    Ok(())
}
