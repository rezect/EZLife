use crate::*;


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
    tokio::time::sleep(Duration::from_millis(200)).await;
    bot.send_message(msg.chat.id, "Привееет, расскажи какая у тебя была сегодня энергия?").await?;
    dialogue.update(State::ReceiveEnergy).await?;
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

pub async fn notion_command(bot: Bot, msg: Message, dialogue: MyDialogue) -> HandlerResult {
    dotenv().ok();
    let notion_acess_url = env::var("NOTION_ACESS_URL").expect("NOTION_ACESS_URL must be set in .env");

    tokio::time::sleep(Duration::from_millis(200)).await;
    bot.send_message(msg.chat.id, "Отлично, давай добавим интеграцию с Notion!").await?;
    tokio::time::sleep(Duration::from_millis(200)).await;
    let ask_to_url = format!("Мне от тебя нужен токен, который ты получишь по ссылке: [*тык*]({})", notion_acess_url);
    bot.send_message(msg.chat.id, ask_to_url)
        .parse_mode(ParseMode::MarkdownV2)
        .await?;
    dialogue.update(State::GetNotionCode).await?;
    Ok(())
}
