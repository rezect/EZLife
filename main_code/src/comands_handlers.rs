use teloxide::{
    prelude::*,
    utils::command::BotCommands,
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
    bot.send_message(msg.chat.id, "Какие эмоции ты хочешь добавить?").await?;
    // Реализация добавления эмоций в файлик
    dialogue.update(State::ReceiveAgree).await?;
    Ok(())
}
