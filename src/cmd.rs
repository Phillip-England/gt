
use crate::{ast::{self, new_ast}, err, file_manager, lexer::GenericLex, tokenizer::{Token, tokenize}};



pub fn help() {
    println!("gt - a agent-first scripting runtime
gt help
gt run <file-path>");
}

pub fn run(filepath: String) -> Result<(), err::AppErr> {
    
    let content: String;
    match file_manager::read_file(filepath) {
        Ok(source_content) => {
            content = source_content;
        },
        Err(err) => {
            return Err(err::AppErr::FileManager(err));
        }
    };

    let toks = tokenize(content)?;
    let ast = new_ast(toks)?;
    println!("{:?}", ast);
    Ok(())
}




