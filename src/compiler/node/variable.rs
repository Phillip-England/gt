use crate::{
    app_err,
    compiler::{
        lexer::Lexer,
        parser::{DataType, ParserErr, stringify_data_type},
        tokenizer::AdvancedToken,
    },
    err::{AppErr, AppErrKind},
};

#[derive(Clone, Debug)]
pub struct Variable {
    pub toks: Vec<AdvancedToken>,
    pub data_type: DataType,
    pub data_type_str: String,
    pub name: String,
    pub value: String,
}

impl Variable {
    pub fn new(toks: Vec<AdvancedToken>) -> Result<Variable, AppErr> {
        let toks_clone = toks
            .iter()
            .map(|t| {
                return t.clone();
            })
            .collect();
        let mut l: Lexer<AdvancedToken> = Lexer::new(toks_clone);
        let var_data_type: DataType;
        let var_name: String;
        let tok1 = l.peek(1);
        match tok1 {
            AdvancedToken::VariableName(name) => {
                var_name = name.clone();
            }
            _ => {
                return Err(app_err!(AppErrKind::Parser(ParserErr::MalformedVariable(
                    String::from("expected 1st token from 'let' keyword to be of type Indicator")
                ))));
            }
        }
        let tok2 = l.peek(2);
        if !matches!(tok2, AdvancedToken::Colon) {
            return Err(app_err!(AppErrKind::Parser(ParserErr::MalformedVariable(
                String::from("expected 2nd token from 'let' keyword to be of type Colon")
            ))));
        }
        let tok3 = l.peek(3);
        match tok3 {
            AdvancedToken::Indicator(s) => {
                var_data_type = DataType::Custom(s);
            }
            AdvancedToken::KeywordBool => {
                var_data_type = DataType::Bool;
            }
            AdvancedToken::KeywordNum => {
                var_data_type = DataType::Num;
            }
            AdvancedToken::KeywordStr => {
                var_data_type = DataType::Str;
            }
            _ => {
                return Err(app_err!(AppErrKind::Parser(ParserErr::MalformedVariable(
                    String::from(
                        "expected 3rd token from 'let' keyword to be of one of the following types: Indicator, Str, Bool, or Num"
                    )
                ))));
            }
        }
        let tok4 = l.peek(4);
        if !matches!(tok4, AdvancedToken::OperatorAssignment) {
            return Err(app_err!(AppErrKind::Parser(ParserErr::MalformedVariable(
                String::from(
                    "expected 4th token from 'let' keyword to be of type OperatorAssignment"
                )
            ))));
        }
        // this is where the value should begin
        l.next_by(5);
        l.mark();
        // ensuring we have a semicolon
        loop {
            let tok = l.item();
            if matches!(tok, AdvancedToken::SemiColon) {
                break;
            }
            if l.at_end() {
                return Err(app_err!(AppErrKind::Parser(ParserErr::MalformedVariable(
                    String::from("failed to located a semicolon for variable")
                ))));
            }
            l.next();
        }
        // value extraction
        let value_toks = l.collect(l.marked_pos(), l.pos());
        let value: String = value_toks
            .into_iter()
            .filter_map(|t| match t {
                AdvancedToken::PromptValue(s) => Some(s),
                _ => return None,
            })
            .collect();

        let mut data_type_str = stringify_data_type(&var_data_type);


        return Ok(Variable {
            toks: toks,
            data_type: var_data_type,
            data_type_str: data_type_str,
            name: var_name,
            value: value,
        });
    }
}
