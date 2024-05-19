use crate::*;


pub async fn callback_handler(bot: Bot, dialogue: MyDialogue, q: CallbackQuery) -> HandlerResult {
    
        tokio::time::sleep(Duration::from_millis(400)).await;
        if let Some(energy) = q.data {
        let text = format!("Твоя энергия: {energy}");

        bot.answer_callback_query(q.id).await?;

        if let Some(Message { id, chat, .. }) = q.message {

            tokio::time::sleep(Duration::from_millis(200)).await;
            bot.edit_message_text(chat.id, id, text).await?;
            
            tokio::time::sleep(Duration::from_millis(400)).await;
            match energy.as_str() {
                "Низкая" => {
                    bot.send_message(chat.id, "Ничего страшного, это нормально бро").await?;
                    tokio::time::sleep(Duration::from_millis(200)).await;
                    bot.send_message(chat.id, "Теперь расскажи о своих чувствах за сегодня").await?;
                    dialogue.update(State::ReceiveEmotions { energy: String::from("Низкая энергия") }).await?;
                }
                "Средняя" => {
                    bot.send_message(chat.id, "Главное во всем держать золотую середину ;)").await?;
                    tokio::time::sleep(Duration::from_millis(200)).await;
                    bot.send_message(chat.id, "Теперь расскажи о своих чувствах за сегодня").await?;
                    dialogue.update(State::ReceiveEmotions { energy: String::from("Средняя энергия") }).await?;
                }
                "Высокая" => {
                    bot.send_message(chat.id, "Сегодня позитивненький день, получается :)").await?;
                    tokio::time::sleep(Duration::from_millis(200)).await;
                    bot.send_message(chat.id, "Теперь расскажи о своих чувствах за сегодня").await?;
                    dialogue.update(State::ReceiveEmotions { energy: String::from("Высокая энергия") }).await?;
                }
                _ => {
                    bot.send_message(chat.id, "Выберите энергию из предложенных вариантов").await?;
                    dialogue.update(State::EnergyError).await?;
                }
            }

        } else if let Some(id) = q.inline_message_id {
            bot.edit_message_text_inline(id, text).await?;
        }
        
        log::info!("User energy: {}", energy);
        
    }

    Ok(())
}

pub async fn make_keyboard_energy() -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

    let debian_versions = [
        "Низкая", "Средняя", "Высокая",
    ];

    tokio::time::sleep(Duration::from_millis(500)).await;

    for versions in debian_versions.chunks(3) {
        let row = versions
            .iter()
            .map(|&version| InlineKeyboardButton::callback(version.to_owned(), version.to_owned()))
            .collect();

        keyboard.push(row);
    }

    InlineKeyboardMarkup::new(keyboard)
}

pub async fn callback_handler_not_correct_state(bot: Bot, q: CallbackQuery) -> HandlerResult {
    
    tokio::time::sleep(Duration::from_millis(400)).await;
    if let Some(energy) = q.data {
    let text = format!("Твоя энергия: {energy}");

    bot.answer_callback_query(q.id).await?;

    if let Some(Message { id, chat, .. }) = q.message {
        bot.edit_message_text(chat.id, id, text).await?;
    } else if let Some(id) = q.inline_message_id {
        bot.edit_message_text_inline(id, text).await?;
    }
    
    log::info!("User energy: {}", energy);
    
}

Ok(())
}