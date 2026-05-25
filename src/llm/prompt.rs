use crate::err::ErrApp;


#[derive(Debug, Clone)]
pub struct Prompt {
    name: String,
    text: String
}


impl Prompt {

    pub fn new(name: String, text: String) -> Prompt {
        return Prompt{
            name,
            text,
        }
    }

}