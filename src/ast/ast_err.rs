

pub enum AstErr {
    MissingOpeningCurlyBrace(String),
    ExpectedIndicatorToken(String),
    MalformedDataType(String),
    MalformedVariable(String),
    MissingSemiColon(String),
}


pub fn handle_ast_err(err: AstErr) {
    match err {
        AstErr::MissingSemiColon(s) => {
            eprintln!("missing semicolon: {}", s);
        },
        AstErr::MissingOpeningCurlyBrace(s) => {
            eprintln!("missing opening curly brace: {}", s);
        },
        AstErr::ExpectedIndicatorToken(s) => {
            eprintln!("expected indicator token but did not find one: {}", s);
        },
        AstErr::MalformedDataType(s) => {
            eprintln!("unexpected data type structure encountered: {}", s);
        },
        AstErr::MalformedVariable(s) => {
            eprintln!("AST ERR: malformed variable: {}", s);
        }
    }
}