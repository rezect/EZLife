use teloxide::{
    prelude::*, types::InputFile, utils::command::BotCommands
};
use std::path::Path;
use std::io::Write;
use std::fs::File;

use crate::{Command, State, MyDialogue, HandlerResult};

pub async fn help_handler(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
    Ok(())
}

pub async fn restart_handler(bot: Bot, msg: Message, dialogue: MyDialogue) -> HandlerResult {
    let chat_id = msg.chat.id.to_string();
    let user_name = msg.from().unwrap().username.to_owned().unwrap_or(String::from("NoName"));
    let path_str = format!("user_data/{}", chat_id);
    let path = Path::new(&path_str);
    if !path.exists() {
        let mut file = File::create(&path)?;
        writeln!(file, "Start documentation! Nickname - {}", user_name)?;
    }
    bot.send_message(msg.chat.id, "Привет, готов поговорить о прошедшем дне? ;)").await?;
    dialogue.update(State::ReceiveAgree).await?;
    Ok(())
}

pub async fn add_reflection_handler(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    let chat_id = msg.chat.id.to_string();
    let path_str = format!("user_conf/{}", chat_id);
    if Path::new(&path_str).exists() {
        bot.send_message(msg.chat.id, "Для этой функции нужна интеграция с Notion, /changedbid").await?;
    } else {
        bot.send_message(msg.chat.id, "Я всегда готов тебя выслушать. Давай, рассказывай!").await?;
        dialogue.update(State::AddNewReflection).await?;
    }
    Ok(())
}

pub async fn send_user_data(bot: Bot, msg: Message, dialogue: MyDialogue) -> HandlerResult {
    let chat_id = msg.chat.id.to_string();
    let input_file = InputFile::file(format!("user_data/{}", chat_id));
    let caption = "Ваши данные, мюсьё:";
    bot.send_document(dialogue.chat_id(), input_file)
        .caption(caption.to_string())
        .send()
        .await?;
    Ok(())
}

pub async fn delete_all_data(bot: Bot, dialogue: MyDialogue) -> HandlerResult {
    bot.send_message(dialogue.chat_id(), "Вы уверены что хотите удалить свои заметки локально?\nВернуть будет пока еще нельзя, но скоро добавлю.").await?;
    dialogue.update(State::DeleteAllUserData).await?;
    Ok(())
}

pub async fn sleep_handler(bot: Bot, dialogue: MyDialogue) -> HandlerResult {
    bot.send_message(dialogue.chat_id(), "Перехожу в спящий режим.").await?;
    dialogue.update(State::Waiting).await?;
    Ok(())
}

pub async fn change_db_id(msg: Message, bot: Bot, dialogue: MyDialogue) -> HandlerResult {
    bot.send_message(msg.chat.id, "Введите свою ссылку на базу данных").await?;
    dialogue.update(State::ReceiveNotionInfo).await?;
    Ok(())
}