use crate::{app_err, compiler::{lexer::{Lexer}, node::Variable, parser::ParserErr, tokenizer::AdvancedToken}, err::{AppErr, AppErrKind}};

pub fn parse_variable(l: &mut Lexer<AdvancedToken>) -> Result<Variable, AppErr> {
    l.mark();
    loop {
        if matches!(l.item(), AdvancedToken::SemiColon) {
            break;
        }
        if l.at_end() {
            return Err(app_err!(AppErrKind::Parser(ParserErr::MissingSemiColon(String::from("could not locate a semicolon for 'let' token")))));   
        }
        l.next();
    }
    l.next();
    let node_var_toks = l.collect(l.marked_pos(), l.pos());
    let node_var = Variable::new(node_var_toks)?;
    l.prev();
    Ok(node_var)
}