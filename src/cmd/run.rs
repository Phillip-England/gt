use crate::{ast::new_ast, err::AppErr, file_manager, tokenizer::tokenize};


pub fn run(filepath: String) -> Result<(), AppErr> {
    
    let content: String;
    match file_manager::read_file(filepath) {
        Ok(source_content) => {
            content = source_content;
        },
        Err(err) => {
            return Err(AppErr::FileManager(err));
        }
    };

    let toks = tokenize(content)?;
    // println!("{:?}", toks);
    let ast = new_ast(toks)?;
    println!("{:?}", ast);
    Ok(())
}