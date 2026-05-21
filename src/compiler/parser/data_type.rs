#[derive(Clone, Debug)]
pub enum DataType {
    Str,
    Num,
    Bool,
    Custom(String),
    Array(Box<DataType>)
}

pub fn stringify_data_type(dt: &DataType) -> String {
    match dt {
        DataType::Str => "str".to_string(),
        DataType::Num => "num".to_string(),
        DataType::Bool => "bool".to_string(),
        DataType::Custom(s) => s.clone(),
        // If it's an array, look inside the box, convert it to a string, and append "[]"
        DataType::Array(inner_box) => {
            let inner_str = stringify_data_type(&**inner_box); 
            format!("{}[]", inner_str)
        }
    }
}