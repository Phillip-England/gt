use crate::{app_err, compiler::{lexer::Lexer, node::{self, DataStruct}, parser::ParserErr, tokenizer::AdvancedToken}, err::{AppErr, AppErrKind}};





pub fn parse_data_struct(l: &mut Lexer<AdvancedToken>) -> Result<DataStruct, AppErr> {
    let mut is_struct_keyword = false;
    if matches!(l.peek(1), AdvancedToken::Indicator(_)) {
        if matches!(l.peek(2), AdvancedToken::OpenedCurlyBrace) {
            if matches!(l.peek(3), AdvancedToken::Indicator(_)) {
                is_struct_keyword = true;
            }
        }
    }
    if !is_struct_keyword {
        return Err(app_err!(AppErrKind::Ast(ParserErr::MissingOpeningCurlyBrace(String::from("expected to find opening curly brace after data keyword but failed to find it"))))) 
    }
    l.mark();
    loop {
        if matches!(l.item(), AdvancedToken::SemiColon) {
            break;
        }
        if l.at_end() {
            return Err(app_err!(AppErrKind::Ast(ParserErr::MissingSemiColon(String::from("expected to find a closing curly brace for our data keyword but failed to find it"))))) 
        }
        l.next();
    }
    l.next();
    let node_data_type_toks = l.collect(l.marked_pos(), l.pos());
    l.prev();
    let node_data_struct = node::DataStruct::new(node_data_type_toks)?;
    Ok(node_data_struct)
}