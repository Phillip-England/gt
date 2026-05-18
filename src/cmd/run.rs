use crate::{ast::new_ast, err::AppErr, file_manager::{self, read_to_string}, lexer::GenericLex, tokenizer::{Token, tokenize}};


pub fn run(filepath: String) -> Result<(), AppErr> {
    let content = read_to_string(filepath)?;
    let chars: Vec<char> = content.chars().collect();

    let tks: Vec<Token> = vec![];
    let mut l: GenericLex<char> = GenericLex::new(chars);
    loop {

        if l.at_end() {
            break;
        }
        l.next();
    }


    let toks = tokenize(content)?;
    // println!("{:?}", toks);
    let ast = new_ast(toks)?;
    // println!("{:?}", ast);
    Ok(())
}