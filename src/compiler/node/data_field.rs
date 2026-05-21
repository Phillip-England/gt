use crate::compiler::parser::DataType;

#[derive(Clone, Debug)]
pub struct DataField {
    pub name: String,
    pub data_type: DataType,
}

impl DataField {
    pub fn new(name: String, data_type: DataType) -> DataField {
        return DataField { name, data_type };
    }
}
