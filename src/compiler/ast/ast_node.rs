use crate::{compiler::ast::{NodeDataType, NodeVariable}, compiler::tokenizer::AdvancedToken};







#[derive(Debug, Clone)]
pub enum AstNode {
    DataType(NodeDataType),
    Variable(NodeVariable),
}


pub fn new_ast_node(_: AdvancedToken) -> Result<(), ()> {
    return Ok(())
}