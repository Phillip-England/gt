use crate::{
    compiler::{parser::parse_ast, tokenizer::tokenize},
    err::{ErrApp},
    interpreter::interpret_ast,
    io::read_to_string,
};

pub fn run(filepath: String, model: String) -> Result<(), ErrApp> {
    let content = read_to_string(filepath)?;
    let toks = tokenize(content)?;
    // println!("{:?}", toks);
    let ast = parse_ast(toks)?;
    // println!("{:?}", ast);
    let _ = interpret_ast(ast, model)?;
    Ok(())
}
