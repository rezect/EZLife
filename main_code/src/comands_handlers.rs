use crate::*;


pub async fn help_command(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
    Ok(())
}

pub async fn new_day_command(bot: Bot, msg: Message, dialogue: MyDialogue) -> HandlerResult {
    if !is_notion_integration_exist(msg.chat.id.to_string()).await {
        bot.send_message(msg.chat.id, "Эта функция пока доступна только с Notion.").await?;
        dialogue.update(State::Waiting).await?;
        return Ok(());
    }
    tokio::time::sleep(Duration::from_millis(200)).await;
    bot.send_message(msg.chat.id, "Привееет, расскажи какая у тебя была сегодня энергия?").await?;
    dialogue.update(State::ReceiveEnergy).await?;
    Ok(())
}

pub async fn sleep_command(bot: Bot, dialogue: MyDialogue) -> HandlerResult {
    bot.send_message(dialogue.chat_id(), "Перехожу в спящий режим.").await?;
    dialogue.update(State::Waiting).await?;
    Ok(())
}

pub async fn notion_command(bot: Bot, msg: Message, dialogue: MyDialogue) -> HandlerResult {
    if !is_notion_integration_exist(msg.chat.id.to_string()).await {
        bot.send_message(msg.chat.id, "Эта функция пока доступна только с Notion.").await?;
        dialogue.update(State::Waiting).await?;
        return Ok(());
    }

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

pub async fn note_command(bot: Bot, msg: Message, dialogue: MyDialogue) -> HandlerResult {
    if !is_notion_integration_exist(msg.chat.id.to_string()).await {
        bot.send_message(msg.chat.id, "Эта функция пока доступна только с Notion.").await?;
        dialogue.update(State::Waiting).await?;
        return Ok(());
    }
    bot.send_message(msg.chat.id, "Расскажи что-нибудь интересное ;)").await?;
    dialogue.update(State::NoteHelper).await?;
    Ok(())
}

pub async fn note_helper(bot: Bot, msg: Message, dialogue: MyDialogue) -> HandlerResult {
    match msg.text() {
        Some(note_info) => {
            if notion_reflection_shema(note_info, msg.chat.id.to_string()).await.status().is_success() {
                bot.send_message(msg.chat.id, "Отлично, записал!\nЕсли еще будет что рассказать - пиши (/note).").await?;
            } else {
                bot.send_message(msg.chat.id, "Не получилось записать в Notion :( Что-то пошло не так...").await?;
            }
        }
        _ => {
            bot.send_message(dialogue.chat_id(), "Я не понял твой ответ. Отправь мне что-нибудь... текстовое").await?;
        }
    }
    dialogue.update(State::Waiting).await?;
    Ok(())
}
