use crate::*;


pub async fn add_new_to_notion(
    (energy, emotions, reflection, rate, cur_date, chat_id): (String, String, String, u32, String, String)
) -> HandlerResult {    
    let mut data_file = File::open(format!("user_conf/{}", chat_id)).expect("File not found");
    let mut database_id = String::new();
    data_file.read_to_string(&mut database_id).expect("File reading failed");
    database_id.pop();

    // Получаем дату в формате "4 апр."
    let local_date = Local::now();
    let month_names = [
        "янв.", "фев", "марта", "апр.", "мая", "июня",
        "июля", "авг.", "сент.", "окт.", "ноября", "дек."
    ];
    let month_number = local_date.month() as usize;
    let month_name = month_names[month_number - 1];
    let day = local_date.day();

    let response = notion_shema_new_page((energy, emotions, reflection, rate, cur_date), (day, month_name, database_id)).await;

    if response.status().is_success() {
        // Получаем тело ответа как строку
        let body = response.text().await?;
        log::info!("Ответ сервера: {}", body);
    } else {
        log::error!("Ошибка: {:?}", response);
    }

    Ok(())
}
