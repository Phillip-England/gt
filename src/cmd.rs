
use crate::{err, file_manager, lexer::{GenericLex}, tokenizer::{Token, tokenize}};


pub fn help() {
    println!("gt - a agent-first scripting runtime
gt help
gt run <file-path>");
}

pub fn run(filepath: String) -> Result<(), err::AppErr> {
    
    let content: String;
    match file_manager::load_source_file(filepath) {
        Ok(source_content) => {
            content = source_content;
        },
        Err(err) => {
            return Err(err::AppErr::FileManager(err));
        }
    };

    let toks = tokenize(content)?;
    let ast_result = new_ast(toks)?;
    Ok(())
}
#[derive(Debug, Clone)]
enum AstNode {
    DataType(String),
}

#[derive(Debug, Clone)]
struct Ast {
    head: AstNode
}

fn new_ast(toks: Vec<Token>) -> Result<Ast, err::AppErr> {
    
    let head: AstNode;


    let mut l: GenericLex<Token> = GenericLex::new(toks);
    loop {
        if l.at_end() {
            break;
        }
        let tok = l.item();
        match tok {
            Token::Colon => {

            },
            Token::Indicator(s) => {

            },
            Token::KeywordData => {

            },
            Token::KeywordNum => {

            },
            Token::KeywordStr => {

            },
            Token::OperatorAssignment => {

            },
            Token::PromptEnd => {

            },
            Token::PromptName(s) => {

            },
            Token::PromptStart => {

            },
            Token::PromptText(s) => {

            },
            Token::SymbolClosedCurlyBrace => {

            },
            Token::SymbolOpenedCurlyBrace => {

            }

        }

        
        println!("{:?}", l.item());
        l.next();
    }

    head = AstNode::DataType(String::from("yo"));
    let ast: Ast = Ast {
        head: head,
    };

    return Ok(ast);
}


fn new_ast_node(tok: Token) -> Result<(), ()> {
    return Ok(())
}






