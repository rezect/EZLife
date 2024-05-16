use crate::*;


pub fn add_str_to_file(path: String, data: String, name_of_string: String) -> std::io::Result<()> {

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(path)?;
    writeln!(file, "{}: {}", name_of_string, data)?;

    Ok(())
}

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
