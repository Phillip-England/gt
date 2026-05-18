use crate::{ast::new_ast, err::AppErr, file_manager::{read_to_string}, tokenizer::{tokenize}};

enum State {
    Init,
    AtWordStart,
}

pub fn run(filepath: String) -> Result<(), AppErr> {
    let content = read_to_string(filepath)?;
    let toks = tokenize(content)?;
    let ast = new_ast(toks)?;
    println!("{:?}", ast);
    Ok(())
}