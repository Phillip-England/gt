use std::fs;

use crate::{
    io::ErrIo, loc,
};

pub fn read_to_string(path: String) -> Result<String, ErrIo> {
    let content_result = fs::read_to_string(path.clone());
    let content: String;
    match content_result {
        Ok(str) => {
            content = str;
        }
        Err(_err) => {
            return Err(ErrIo::LoadFileErr { loc: loc!(), path });
        }
    }
    return Ok(content);
}
