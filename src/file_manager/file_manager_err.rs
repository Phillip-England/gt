





#[derive(Debug)]
pub enum FileManagerErr {
    LoadFileErr(String),
}


pub fn handle_file_manager_err(err: FileManagerErr) {
    match err {
        FileManagerErr::LoadFileErr(s) => {
            eprintln!("{}", s);
        }
    }
}