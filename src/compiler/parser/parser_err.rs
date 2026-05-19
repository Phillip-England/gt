

#[derive(Debug)]
pub enum ParserErr {
    MissingOpeningCurlyBrace(String),
    ExpectedIndicatorToken(String),
    MalformedDataType(String),
    MalformedVariable(String),
    MissingSemiColon(String),
}


pub fn handle_ast_err(err: ParserErr) {
    match err {
        ParserErr::MissingSemiColon(s) => {
            eprintln!("missing semicolon: {}", s);
        },
        ParserErr::MissingOpeningCurlyBrace(s) => {
            eprintln!("missing opening curly brace: {}", s);
        },
        ParserErr::ExpectedIndicatorToken(s) => {
            eprintln!("expected indicator token but did not find one: {}", s);
        },
        ParserErr::MalformedDataType(s) => {
            eprintln!("unexpected data type structure encountered: {}", s);
        },
        ParserErr::MalformedVariable(s) => {
            eprintln!("AST ERR: malformed variable: {}", s);
        }
    }
}