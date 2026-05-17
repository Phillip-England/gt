use std::fs;



#[derive(Debug)]
pub enum FileManagerErr {
    LoadFileErr(String),
}

pub fn read_file(path: String) -> Result<String, FileManagerErr> {
    let content_result = fs::read_to_string(path.clone());
    let content: String;
    match content_result {
        Ok(str) => {
            content = str;
        },
        Err(_err) => {
            return Err(FileManagerErr::LoadFileErr(format!("failed to load file at: {}", path)))
        }
    }
    return Ok(content)
}

pub fn handle_file_manager_err(err: FileManagerErr) {
    match err {
        FileManagerErr::LoadFileErr(s) => {
            eprintln!("{}", s);
        }
    }
}
