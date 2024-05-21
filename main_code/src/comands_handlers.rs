use crate::*;


pub async fn help_command(bot: Bot, msg: Message) -> HandlerResult {
    let help_text = "Привет, меня зовут EZLife\\. 😊
Я \\- бот, который будет помогать тебе организовать свой дневник в [Notion](https://www\\.notion\\.so)\\. 🤖

*Я могу:*💪😎
_*1\\. Записать твой день*_ \\(лучше это делать ближе к вечеру\\): 🌆
    В дне сохраняются следующие данные: твоя _энергия_ за день, твои _эмоции_, общий _отзыв о дне_ и _рейтинг дня_ от 0 до 10\\.
_*2\\. Сделать быструю заметку*_: 🚀
    Записать какое\\-нибудь интересное событие или ваши эмоции в данный момент\\.

Все ваши записи будут сохраняться с соответствующими тегами\\. Используя их вы можете сортировать и фильтровать ваши записи в Notion\\. Также вы можете использовать различные виды базы данных Notion, чтобы было удобней разделять заметки и дни\\. В общем делайте как вам будет удобно, поэтому Notion и классный\\. 📝
Бот не сохраняет и не просматривает ваши данные, только сохраняет в вашу базу данных Notion, так что пишите что хотите.
❗️Важно: не меняйте цвет тегов и колонки в вашей базе данных Notion, потому что могут возникнуть проблемы с загрузкой туда ваших записей❗️
Приятного пользования\\! 🌟

*Основные мои команды:*
_*1\\. /notion*_ \\- предоставить доступ к своей странице в Notion, где вы планируете хранить свои дни и заметки\\.
_*2\\. /day*_ \\- записать отзыв о дне\\.
_*3\\. /note*_ \\- сделать быструю заметку\\.
_*4\\. /sleep*_ \\- перейти в спящий режим, где бот будет отвечать в режиме YaGPT \\(Искусственный интеллект от Yandex\\)\\.";
    bot.send_message(msg.chat.id, help_text)
        .parse_mode(ParseMode::MarkdownV2)
        .await?;
    Ok(())
}

pub async fn new_day_command(bot: Bot, msg: Message, dialogue: MyDialogue) -> HandlerResult {
    if !is_notion_integration_exist(msg.chat.id.to_string()).await {
        bot.send_message(msg.chat.id, "Эта функция пока доступна только с Notion.").await?;
        dialogue.update(State::Waiting).await?;
        return Ok(());
    }
    let keyboard = make_keyboard_energy().await;
    bot.send_message(msg.chat.id, "Какая у вас была сегодня энергия?").reply_markup(keyboard).await?;
    dialogue.update(State::EnergyError).await?;
    Ok(())
}

pub async fn sleep_command(bot: Bot, dialogue: MyDialogue) -> HandlerResult {
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
                bot.send_message(msg.chat.id, "Отлично, записал!\nЕсли еще будет что рассказать - пиши /note.").await?;
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
