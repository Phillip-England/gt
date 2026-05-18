use crate::{ast::{AstErr, DataType}, err::AppErr, lexer::GenericLex, tokenizer::AdvancedToken};




#[derive(Clone, Debug)]
pub struct NodeVariable {
    toks: Vec<AdvancedToken>,
    t: DataType,
    name: String,
}

impl NodeVariable {


    pub fn new(toks: Vec<AdvancedToken>) -> Result<NodeVariable, AppErr> {
        let toks_clone = toks.iter().map(|t| {
            return t.clone();
        }).collect();
        let mut l: GenericLex<AdvancedToken> = GenericLex::new(toks_clone);
        let var_data_type: DataType;
        let var_name: String;
        let tok1 = l.peek(1);
        match tok1 {
            AdvancedToken::VariableName(name) => {
                var_name = name.clone();
            },
            _ => {
                return Err(AppErr::Ast(AstErr::MalformedVariable(String::from("expected 1st token from 'let' keyword to be of type Indicator"))));
            }
        }
        let tok2 = l.peek(2);
        if !matches!(tok2, AdvancedToken::Colon) {
            return Err(AppErr::Ast(AstErr::MalformedVariable(String::from("expected 2nd token from 'let' keyword to be of type Colon"))));
        }
        let tok3 = l.peek(3);
        match tok3 {
            AdvancedToken::Indicator(_) => {
                var_data_type = DataType::Custom;
            },
            AdvancedToken::KeywordBool => {
                var_data_type = DataType::Bool;
            },
            AdvancedToken::KeywordNum => {
                var_data_type = DataType::Num;
            },
            AdvancedToken::KeywordStr => {
                var_data_type = DataType::Str;
            }
            _ => {
                return Err(AppErr::Ast(AstErr::MalformedVariable(String::from("expected 3rd token from 'let' keyword to be of one of the following types: Indicator, Str, Bool, or Num"))));
            }
        }
        let tok4 = l.peek(4);
        if !matches!(tok4, AdvancedToken::OperatorAssignment) {
            return Err(AppErr::Ast(AstErr::MalformedVariable(String::from("expected 4th token from 'let' keyword to be of type OperatorAssignment"))))
        }
        // have to find end of variable
        l.mark();
        l.next_by(5);
        loop {
            let tok = l.item();
            if matches!(tok, AdvancedToken::SemiColon) {
                break;
            }
            if l.at_end() {
                return Err(AppErr::Ast(AstErr::MalformedVariable(String::from("failed to located a semicolon for variable"))));
            }
            l.next();
        }
        return Ok(NodeVariable {
            toks: toks,
            t: var_data_type,
            name: var_name,
        })
    }

}