use crate::{
    compiler::{parser::{Ast}, tokenizer::tokenize},
    err::ErrApp,
    interpreter::interpret_ast,
    io::read_to_string,
};

pub fn run(filepath: String, model: String) -> Result<(), ErrApp> {
    let content = read_to_string(filepath)?;
    let toks = tokenize(content)?;
    let ast = Ast::new(toks)?;
    println!("{:?}", ast.vec_prompts);
    // let _ = interpret_ast(ast, model)?;
    Ok(())
}
