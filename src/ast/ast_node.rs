use crate::{ast::{NodeDataType, NodeVariable}, tokenizer::Token};






#[derive(Debug, Clone)]
pub enum AstNode {
    DataType(NodeDataType),
    Variable(NodeVariable),
}


pub fn new_ast_node(tok: Token) -> Result<(), ()> {
    return Ok(())
}