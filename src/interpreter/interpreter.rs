
use std::collections::HashMap;

use crate::{app_err, compiler::{node::{DataStruct, Node, Variable}, parser::{Ast, DataType}, tokenizer::AdvancedToken}, err::{AppErr, AppErrKind}, llm::ollama::{err_if_no_ollama, is_ollama_installed}};

pub fn interpret_ast(ast: Ast) -> Result<(), AppErr> {

    let mut data_struct_map: HashMap<String, DataStruct> = HashMap::new();
    let mut variable_map: HashMap<String, Variable> = HashMap::new();

    // gaining knowledge of available artifacts
    for node in ast.head_nodes {
        match node {
            Node::DataStruct(data_struct) => {
                let key = data_struct.name.clone();
                let exists = data_struct_map.contains_key(&key);
                if exists {
                    return Err(app_err!(AppErrKind::Interpreter(InterpreterErr::StructDuplication(data_struct))));
                }
                data_struct_map.insert(data_struct.name.clone(), data_struct);
            },
            Node::Variable(variable) => {
                let key = variable.name.clone();
                let exists = variable_map.contains_key(&key);
                if exists {
                   return Err(app_err!(AppErrKind::Interpreter(InterpreterErr::VariableDuplication(variable))));  
                }
                variable_map.insert(variable.name.clone(), variable);
            },
        }
    }

    // extracting our our prompts
    let prompts: Vec<Variable> = variable_map.into_iter().filter_map(|(_, variable)| {
        match variable.t {
            DataType::Custom => {
                let mut prompt_text: String = "".to_string();
                let prompt_toks: Vec<&AdvancedToken> = variable.toks.iter().filter(|tok| {
                    match tok {
                        AdvancedToken::PromptValue(s) => {
                            prompt_text = String::from(s);
                            true
                        },
                        _ => {
                            false
                        }  
                    }
                }).collect();
                if prompt_toks.len() == 0 {
                    return None
                } else {
                    return Some(variable)
                }
            },
            _ => { None }
        }

    }).collect(); 

    err_if_no_ollama()?;


    // doing something with the prompts
    prompts.iter().for_each(|p| {
        println!("{}", p.value);
    });

    return Ok(())
}


#[derive(Debug)]
pub enum InterpreterErr {
    StructDuplication(DataStruct),
    VariableDuplication(Variable),
}

pub fn handle_err(err: InterpreterErr) {
    eprintln!("Interpreter Error:");
    match err {
        InterpreterErr::StructDuplication(data_struct) => {
            eprintln!("{}", format!("The data structure named {} was found multiple times", data_struct.name)); 
        },
        InterpreterErr::VariableDuplication(variable) => { 
            eprintln!("{}", format!("The data variable named {} was found declared multiple times", variable.name)); 
        },
    }
}