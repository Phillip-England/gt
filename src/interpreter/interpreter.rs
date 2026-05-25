use std::collections::HashMap;

use reqwest::blocking::Client;

use crate::{
    compiler::{
        node::{DataStruct, Node, Variable},
        parser::{Ast, DataType},
        tokenizer::AdvancedToken, validator::ErrValidator,
    },
    err::ErrApp,
    fail,
    interpreter::ErrInterpreter,
    llm::{
        get_installed_ollama_models,
        ollama::{err_if_no_ollama, stream_prompt},
    },
};

#[derive(Debug)]
pub struct Interpreter {
    pub ast: Ast,
    pub map_structs: HashMap<String, DataStruct>,
    pub map_vars: HashMap<String, DataStruct>
}


pub fn interpret_ast(ast: Ast, model: String) -> Result<(), ErrApp> {

    // err_if_no_ollama()?;

    // let _models = get_installed_ollama_models()?;
    // let client = Client::new();

    // for p in prompts.iter() {
    //     let data_type_struct: &DataStruct = match data_struct_map.get(&p.data_type_str) {
    //         Some(s) => s,
    //         None => {
    //             return fail!(
    //                 ErrValidator::InvalidVariableType,
    //                 "invalid variable type: {}",
    //                 p.data_type_str
    //             );
    //         }
    //     };

    //     let substruct_names =
    //         data_type_struct.get_substruct_names(vec![], &data_struct_map, data_type_struct)?;

    //     println!("{:?}", substruct_names);

    //     let substructs: Vec<&DataStruct> = data_struct_map
    //         .iter()
    //         .filter_map(|(_, data_struct)| {
    //             if substruct_names.contains(&data_struct.name) {
    //                 Some(data_struct)
    //             } else {
    //                 None
    //             }
    //         })
    //         .collect();

    //     println!("{:?}", substructs);

    //     let res = stream_prompt(&client, &model, &p.value, |chunk| {
    //         println!("{:?}", chunk);
    //     })?;

    //     // response validation
    //     println!("{:?}", res.text);
    // }

    Ok(())
}

const JSON_DEF_CURSOR: &str = "DEF_CURSOR";
const JSON_CURSOR: &str = "JSON_CURSOR";

#[derive(Debug)]
pub struct JsonSchema {
    schema: String,
}

impl JsonSchema {
    pub fn new() -> JsonSchema {
        JsonSchema {
            schema: format!("{{ \"$refs\":{{ {} }}, {} }}", JSON_DEF_CURSOR, JSON_CURSOR),
        }
    }
}

#[derive(Debug)]
enum JsonItemType {
    Object,
    String,
    Number,
    Integer,
    Boolean,
    Null,
    Array(Box<JsonItemType>),
}
