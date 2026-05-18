use crate::ast::DataType;





#[derive(Clone, Debug)]
pub struct NodeDataField {
    name: String,
    t: DataType
}

impl NodeDataField {

    pub fn new(name: String, t: DataType) -> NodeDataField {
        return NodeDataField {
            name,
            t,
        }
    }

}