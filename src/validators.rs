use std::{path::Path, str::FromStr};

pub fn is_exists_file(path: String) -> Result<(), String> {
    let path = Path::new(&path);
    if path.is_dir() {
        Err("Input can't be dir".to_string())
    } else if !path.exists() {
        Err("Target file doesn't exists".to_string())
    } else {
        Ok(())
    }
}

pub fn is_positive_integer(data: String) -> Result<(), String> {
    let parse_result = usize::from_str(&data);
    if parse_result.is_err() || parse_result.unwrap() == 0 {
        Err("Input should be positive integer".to_string())
    } else {
        Ok(())
    }
}
