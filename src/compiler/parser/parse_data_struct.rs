use crate::{
    compiler::{
        lexer::Lexer,
        node::{self, DataStruct},
        parser::ErrParser,
        tokenizer::AdvancedToken,
    },
    fail,
};

pub fn parse_data_struct(l: &mut Lexer<AdvancedToken>) -> Result<DataStruct, ErrParser> {
    let mut is_struct_keyword = false;

    if matches!(l.peek(1), AdvancedToken::Indicator(_)) {
        if matches!(l.peek(2), AdvancedToken::OpenedCurlyBrace) {
            if matches!(l.peek(3), AdvancedToken::Indicator(_)) {
                is_struct_keyword = true;
            }
        }
    }

    if !is_struct_keyword {
        return fail!(
            ErrParser::MissingOpeningCurlyBrace,
            "expected to find opening curly brace after data keyword but failed to find it"
        );
    }

    l.mark();

    loop {
        if matches!(l.item(), AdvancedToken::SemiColon) {
            break;
        }

        if l.at_end() {
            return fail!(
                ErrParser::MissingSemiColon,
                "expected semicolon after data structure declaration"
            );
        }

        l.next();
    }

    l.next();

    let node_data_type_toks = l.collect(l.marked_pos(), l.pos());

    l.prev();

    let node_data_struct = node::DataStruct::new(node_data_type_toks)?;

    Ok(node_data_struct)
}