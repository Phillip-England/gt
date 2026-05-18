use std::fs;

use crate::{err::AppErr, file_manager::FileManagerErr};



pub fn read_to_string(path: String) -> Result<String, AppErr> {
    let content_result = fs::read_to_string(path.clone());
    let content: String;
    match content_result {
        Ok(str) => {
            content = str;
        },
        Err(_err) => {
            return Err(AppErr::FileManager(FileManagerErr::LoadFileErr(format!("failed to load file at: {}", path))))
        }
    }
    return Ok(content)
}