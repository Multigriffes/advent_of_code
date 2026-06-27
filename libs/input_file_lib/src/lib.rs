use std::fs;

pub fn get_file_content_to_string(path: &String) -> Result<String, String> {
    let input_file = path.replace('"', "");
    match fs::read_to_string(input_file) {
        Ok(content) => Ok(content),
        Err(error) => Err(format!("{}", error)),
    }
}
