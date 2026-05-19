





#[derive(Debug)]
pub enum IoErr {
    LoadFileErr(String),
}


pub fn handle_io_err(err: IoErr) {
    match err {
        IoErr::LoadFileErr(s) => {
            eprintln!("{}", s);
        }
    }
}