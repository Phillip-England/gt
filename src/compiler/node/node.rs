use crate::compiler::node::{DataStruct, Variable};

#[derive(Debug, Clone)]
pub enum Node {
    DataStruct(DataStruct),
    Variable(Variable),
}
