use crate::*;


pub fn format_str(cur_str: &str) -> String {
    use teloxide::utils::markdown::escape;
    
    return escape(cur_str)
    .replace("\\n", "\n")
    .replace(r"\*\*", "*")
    .replace(r"\*", "*")
    .replace(r"\_", "_")
    .replace(r"\_\_", "__");
}

pub async fn check_or_create_file(path: &str) {
    if !Path::new(path).exists() {
        match OpenOptions::new()
            .create(true)
            .write(true)
            .open(path) {
            Ok(_) => log::trace!("File created successfully."),
            Err(e) => log::error!("Failed to create file: {}", e),
        }
    } else {
        log::trace!("File already exists.");
    }
}

pub async fn is_notion_integration_exist(chat_id: String) -> bool {
    let path1_str = format!("user_tokens/{}", chat_id);
    let path2_str = format!("user_db_ids/{}", chat_id);
    let path1 = Path::new(&path1_str);
    let path2 = Path::new(&path2_str);
    return path1.exists() && path2.exists();
}