pub fn add_str_to_file(path: String, data: String, name_of_string: String) -> std::io::Result<()> {
    use std::fs::OpenOptions;
    use std::io::Write;

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(path)?;
    writeln!(file, "{}: {}", name_of_string, data)?;

    Ok(())
}
