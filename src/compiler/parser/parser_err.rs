#[derive(Debug)]
pub enum ParserErr {
    MissingOpeningCurlyBrace(String),
    ExpectedIndicatorToken(String),
    MalformedDataType(String),
    MalformedVariable(String),
    MissingSemiColon(String),
    MisplacedArrayIndication(String),
    MalformedStruct(String),
    InfiniteStructReference(String),
}

pub fn handle_ast_err(err: ParserErr) {
    match err {
        ParserErr::InfiniteStructReference(s) => {
            eprintln!("infinite struct reference: {}", s)
        },
        ParserErr::MisplacedArrayIndication(s) => {
            eprintln!("array indication in invalid location: {}", s);
        },
        ParserErr::MissingSemiColon(s) => {
            eprintln!("missing semicolon: {}", s);
        }
        ParserErr::MissingOpeningCurlyBrace(s) => {
            eprintln!("missing opening curly brace: {}", s);
        }
        ParserErr::ExpectedIndicatorToken(s) => {
            eprintln!("expected indicator token but did not find one: {}", s);
        }
        ParserErr::MalformedDataType(s) => {
            eprintln!("unexpected data type structure encountered: {}", s);
        }
        ParserErr::MalformedVariable(s) => {
            eprintln!("AST ERR: malformed variable: {}", s);
        },
        ParserErr::MalformedStruct(s) => {
            eprintln!("malformed struct: {}", s);
        }
    }
}
