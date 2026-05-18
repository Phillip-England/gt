use crate::{compiler::ast::new_ast, err::AppErr, file_manager::{read_to_string}, compiler::tokenizer::{tokenize}};

pub fn run(filepath: String) -> Result<(), AppErr> {
    let content = read_to_string(filepath)?;
    let toks = tokenize(content)?;
    let ast = new_ast(toks)?;
    println!("{:?}", ast);
    Ok(())
}