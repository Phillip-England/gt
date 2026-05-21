use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
};

use reqwest::blocking::{Client, Response};
use serde::Deserialize;
use serde_json::json;

use crate::{
    app_err,
    compiler::{
        node::{DataStruct, Node, Variable},
        parser::{Ast, DataType},
        tokenizer::AdvancedToken,
    },
    err::{AppErr, AppErrKind},
    llm::{
        LlmErr, OLLAMA_ADDR, get_installed_ollama_models,
        ollama::{err_if_no_ollama, is_ollama_installed},
        stream_prompt,
    },
};

pub fn interpret_ast(ast: Ast, model: String) -> Result<(), AppErr> {
    let mut data_struct_map: HashMap<String, DataStruct> = HashMap::new();
    let mut variable_map: HashMap<String, Variable> = HashMap::new();

    // gaining knowledge of available artifacts
    for node in ast.head_nodes {
        match node {
            Node::DataStruct(data_struct) => {
                let key = data_struct.name.clone();
                let exists = data_struct_map.contains_key(&key);
                if exists {
                    return Err(app_err!(AppErrKind::Interpreter(
                        InterpreterErr::StructDuplication(data_struct)
                    )));
                }
                data_struct_map.insert(data_struct.name.clone(), data_struct);
            }
            Node::Variable(variable) => {
                let key = variable.name.clone();
                let exists = variable_map.contains_key(&key);
                if exists {
                    return Err(app_err!(AppErrKind::Interpreter(
                        InterpreterErr::VariableDuplication(variable)
                    )));
                }
                variable_map.insert(variable.name.clone(), variable);
            }
        }
    }



    // extracting our our prompts
    let prompts: Vec<Variable> = variable_map
        .into_iter()
        .filter_map(|(_, variable)| match variable.data_type {
            DataType::Custom(_) => {
                let mut prompt_text: String = "".to_string();
                let prompt_toks: Vec<&AdvancedToken> = variable
                    .toks
                    .iter()
                    .filter(|tok| match tok {
                        AdvancedToken::PromptValue(s) => {
                            prompt_text = String::from(s);
                            true
                        }
                        _ => false,
                    })
                    .collect();
                if prompt_toks.len() == 0 {
                    return None;
                } else {
                    return Some(variable);
                }
            }
            _ => None,
        })
        .collect();

    err_if_no_ollama()?;
    let models = get_installed_ollama_models()?;


    let client = Client::new();
    for p in prompts.into_iter() {

        let data_type_struct: &DataStruct = match data_struct_map.get(&p.data_type_str) {
            Some(s) => s,
            None => { 
                return Err(app_err!(AppErrKind::Interpreter(InterpreterErr::InvalidVariableType(p))))
            },
        };


        println!("{:?}", data_type_struct);
        let substructs = data_type_struct.get_substruct_names(vec![]);
        println!("{:?}", substructs);
        // let schema = JsonSchema::new();
        // println!("{:?}", schema);

        // for st in data_struct_map.iter() {


        //     println!("{:?}", st);
        // }




        let res = stream_prompt(&client, &model, &p.value)?;

        // response validation
        println!("{:?}", res.text);
    }
    return Ok(());
}

#[derive(Debug)]
pub struct Interpreter {
    pub ast: Ast
}

impl Interpreter {



}

#[derive(Debug)]
pub enum InterpreterErr {
    StructDuplication(DataStruct),
    VariableDuplication(Variable),
    InvalidVariableType(Variable),
}

pub fn handle_err(err: InterpreterErr) {
    eprintln!("Interpreter Error:");
    match err {
        InterpreterErr::InvalidVariableType(variable) => {

        },
        InterpreterErr::StructDuplication(data_struct) => {
            eprintln!(
                "{}",
                format!(
                    "The data structure named {} was found multiple times",
                    data_struct.name
                )
            );
        }
        InterpreterErr::VariableDuplication(variable) => {
            eprintln!(
                "{}",
                format!(
                    "The data variable named {} was found declared multiple times",
                    variable.name
                )
            );
        }
    }
}





const JSON_DEF_CURSOR: &str = "DEF_CURSOR";
const JSON_CURSOR: &str = "JSON_CURSOR";

#[derive(Debug)]
pub struct JsonSchema {
    schema: String
}

impl JsonSchema {

    pub fn new() -> JsonSchema {
        let s = JsonSchema{
            schema: format!("{{ \"$refs\":{{ {} }}, {} }}", JSON_DEF_CURSOR, JSON_CURSOR),
        };
        s
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
    Array(Box<JsonItemType>)
}

