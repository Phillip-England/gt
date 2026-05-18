use crate::{ast::{AstErr, AstNode, DataType, NodeDataField, NodeDataType, NodeVariable}, err::AppErr, lexer::GenericLex, tokenizer::Token};





#[derive(Debug, Clone)]
pub struct Ast {
    head_nodes: Vec<AstNode> 
}

pub fn new_ast(toks: Vec<Token>) -> Result<Ast, AppErr> {
    // println!("{:?}", toks);
    
    let mut ast: Ast = Ast {
        head_nodes: vec![],
    };


    let mut l: GenericLex<Token> = GenericLex::new(toks);
    loop {
        let tok = l.item();
        match tok {
            Token::Colon => {

            },
            Token::SemiColon => {

            },
            Token::Indicator(s) => {

            },
            Token::EndOfFile => {

            },
            Token::KeywordLet => {
                l.mark();
                loop {
                    if matches!(l.item(), Token::SemiColon) {
                        break;
                    }
                    if l.at_end() {
                        return Err(AppErr::Ast(AstErr::MissingSemiColon(String::from("could not locate a semicolon for 'let' token"))));   
                    }
                    l.next();
                }
                l.next();
                let node_var_toks = l.collect(l.marked_pos(), l.pos());
                let node_var = NodeVariable::new(node_var_toks)?;
                ast.head_nodes.push(AstNode::Variable(node_var));
            },
            Token::KeywordData => {
                let mut is_data_keyword = false;
                if matches!(l.peek(1), Token::Indicator(s)) {
                    if matches!(l.peek(2), Token::OpenedCurlyBrace) {
                        if matches!(l.peek(3), Token::Indicator(s2)) {
                            is_data_keyword = true;
                        }
                    }
                }
                if !is_data_keyword {
                    return Err(AppErr::Ast(AstErr::MissingOpeningCurlyBrace(String::from("expected to find opening curly brace after data keyword but failed to find it")))) 
                }
                l.mark();
                loop {
                    if matches!(l.item(), Token::SemiColon) {
                        break;
                    }
                    if l.at_end() {
                        return Err(AppErr::Ast(AstErr::MissingSemiColon(String::from("expected to find a closing curly brace for our data keyword but failed to find it")))) 
                    }
                    l.next();
                }
                l.next();
                let node_data_type_toks = l.collect(l.marked_pos(), l.pos());
                let node_data_type = NodeDataType::new(node_data_type_toks)?;
                ast.head_nodes.push(AstNode::DataType(node_data_type));
                l.prev();
            },
            Token::KeywordNum => {

            },
            Token::KeywordStr => {

            },
            Token::KeywordBool => {

            },
            Token::OperatorAssignment => {

            },
            Token::PromptEnd => {

            },
            Token::VariableName(_) => {

            },
            Token::PromptStart => {

            },
            Token::PromptText(_) => {

            },
            Token::ClosedCurlyBrace => {

            },
            Token::OpenedCurlyBrace => {

            }

        }

        if l.at_end() {
            break;
        }
        l.next();
    }



    return Ok(ast);
}
