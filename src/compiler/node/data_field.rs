use crate::compiler::parser::DataType;

#[derive(Clone, Debug)]
pub struct DataField {
    name: String,
    t: DataType
}

impl DataField {

    pub fn new(name: String, t: DataType) -> DataField {
        return DataField {
            name,
            t,
        }
    }

}