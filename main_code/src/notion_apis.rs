use crate::*;


pub async fn add_new_to_notion(
    (energy, emotions, reflection, rate, cur_date, chat_id, bot): (String, String, String, u32, String, String, Bot)
) -> HandlerResult {
    // Получаем дату в формате "4 апр."
    let local_date = Local::now();
    let month_names = [
        "янв.", "фев.", "марта", "апр.", "мая", "июня",
        "июля", "авг.", "сент.", "окт.", "ноября", "дек."
    ];
    let month_number = local_date.month() as usize;
    let month_name = month_names[month_number - 1];
    let day = local_date.day();

    let response = notion_shema_new_page((energy, emotions, reflection, rate, cur_date), (day, month_name, chat_id.clone())).await;

    if response.status().is_success() {
        // Получаем тело ответа как строку
        let body = response.text().await?;
        log::info!("Ответ сервера: {}", body);
    } else {
        bot.send_message(ChatId(chat_id.parse::<i64>().unwrap()), "Ошибка при записи в Notion\nУбедитесь, что вы не удаляли свойства (properties) базы данных.\nОна должна выглядеть так:").await?;
        let mut photo = PathBuf::new();
        photo.push("images/properties_error.jpg");
        bot.send_photo(ChatId(chat_id.parse::<i64>().unwrap()), InputFile::file(photo)).await?;
        log::error!("Ошибка: {:?}", response);
    }

    Ok(())
}
