use crate::{
    compiler::{
        lexer::Lexer,
        node::Node,
        parser::{parse_data_struct, parse_variable},
        tokenizer::AdvancedToken,
    },
    err::AppErr,
};

#[derive(Debug, Clone)]
pub struct Ast {
    pub head_nodes: Vec<Node>,
}

pub fn parse_ast(toks: Vec<AdvancedToken>) -> Result<Ast, AppErr> {
    // println!("{:?}", toks);

    let mut ast: Ast = Ast { head_nodes: vec![] };

    let mut l: Lexer<AdvancedToken> = Lexer::new(toks);
    loop {
        let tok = l.item();
        match tok {
            AdvancedToken::Colon => {}
            AdvancedToken::SemiColon => {}
            AdvancedToken::Indicator(_) => {}
            AdvancedToken::EndOfFile => {}
            AdvancedToken::KeywordLet => {
                let node_variable = parse_variable(&mut l)?;
                ast.head_nodes.push(Node::Variable(node_variable));
            }
            AdvancedToken::KeywordStruct => {
                let node_data_struct = parse_data_struct(&mut l)?;
                ast.head_nodes.push(Node::DataStruct(node_data_struct));
            }
            AdvancedToken::KeywordNum => {}
            AdvancedToken::KeywordStr => {}
            AdvancedToken::KeywordBool => {}
            AdvancedToken::OperatorAssignment => {}
            AdvancedToken::PromptEnd => {}
            AdvancedToken::VariableName(_) => {}
            AdvancedToken::PromptStart => {}
            AdvancedToken::PromptValue(_) => {}
            AdvancedToken::ClosedCurlyBrace => {}
            AdvancedToken::OpenedCurlyBrace => {}
            AdvancedToken::DoubleQuote => {}
            AdvancedToken::StrValue(s) => {}
        }

        if l.at_end() {
            break;
        }
        l.next();
    }

    return Ok(ast);
}
