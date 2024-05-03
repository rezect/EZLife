use crate::yagpt_is_user_ready;


pub async fn is_user_ready_ai(
    msg_text: String
) -> char {
    let response = yagpt_is_user_ready(msg_text).await;
    let response_char = response.chars().next().unwrap_or('N');

    return response_char;
}