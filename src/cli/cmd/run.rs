use crate::{compiler::{parser::parse_ast, tokenizer::tokenize}, err::AppErr, interpreter::interpret_ast, io::read_to_string};

pub fn run(filepath: String) -> Result<(), AppErr> {
    let content = read_to_string(filepath)?;
    let toks = tokenize(content)?;
    // println!("{:?}", toks);
    let ast = parse_ast(toks)?;
    // println!("{:?}", ast);
    let _ = interpret_ast(ast)?;
    Ok(())
}