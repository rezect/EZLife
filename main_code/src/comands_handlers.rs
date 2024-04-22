use teloxide::{
    prelude::*,
    utils::command::BotCommands,
    types::InputFile,
};

use crate::{Command, State, MyDialogue, HandlerResult};

pub async fn help_handler(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
    Ok(())
}

pub async fn restart_handler(bot: Bot, msg: Message, dialogue: MyDialogue) -> HandlerResult {
    bot.send_message(msg.chat.id, "Привет, готов поговорить о прошедшем дне? ;)").await?;
    dialogue.update(State::ReceiveAgree).await?;
    Ok(())
}

pub async fn add_emotions_handler(bot: Bot, msg: Message, dialogue: MyDialogue) -> HandlerResult {
    bot.send_message(msg.chat.id, "Пока дорабатывается;)").await?;
    // Реализация добавления эмоций в файлик
    dialogue.update(State::Start).await?;
    Ok(())
}

pub async fn send_user_data(bot: Bot, msg: Message, dialogue: MyDialogue) -> HandlerResult {
    let chat_id = msg.chat.id.to_string();
    let input_file = InputFile::file(format!("user_data/{}", chat_id));
    let caption = "Ваши данные:";
    bot.send_document(dialogue.chat_id(), input_file)
        .caption(caption.to_string())
        .send()
        .await?;
    Ok(())
}

pub async fn delete_all_data(bot: Bot, dialogue: MyDialogue) -> HandlerResult {
    bot.send_message(dialogue.chat_id(), "Вы уверены что хотите удалить свои заметки?\nВернуть будет пока еще нельзя, но скоро добавлю.").await?;
    dialogue.update(State::DeleteAllUserData).await?;
    Ok(())
}