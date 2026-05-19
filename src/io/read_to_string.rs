use std::fs;

use crate::{app_err, err::{AppErr, AppErrKind}, io::IoErr};



pub fn read_to_string(path: String) -> Result<String, AppErr> {
    let content_result = fs::read_to_string(path.clone());
    let content: String;
    match content_result {
        Ok(str) => {
            content = str;
        },
        Err(_err) => {
            return Err(app_err!(AppErrKind::Io(IoErr::LoadFileErr(format!("failed to load file at: {}", path)))))
        }
    }
    return Ok(content)
}