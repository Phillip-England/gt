use crate::{
    compiler::{
        lexer::Lexer,
        node::Variable,
        parser::ErrParser,
        tokenizer::AdvancedToken,
    },
    fail,
};

pub fn parse_variable(l: &mut Lexer<AdvancedToken>) -> Result<Variable, ErrParser> {
    l.mark();

    loop {
        if matches!(l.item(), AdvancedToken::SemiColon) {
            break;
        }

        if l.at_end() {
            return fail!(
                ErrParser::MissingSemiColon,
                "missing semicolon for let token"
            );
        }

        l.next();
    }

    l.next();

    let node_var_toks = l.collect(l.marked_pos(), l.pos());
    let node_var = Variable::new(node_var_toks)?;

    l.prev();

    Ok(node_var)
}