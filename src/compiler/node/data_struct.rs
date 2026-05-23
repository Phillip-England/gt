use std::collections::HashMap;

use crate::{
    app_err,
    compiler::{
        lexer, node::DataField, parser::{DataType, ParserErr}, tokenizer::AdvancedToken
    },
    err::{AppErr, AppErrKind},
    interpreter::{InterpreterErr, err_invalid_struct_access},
};

#[derive(Clone, Debug)]
pub struct DataStruct {
    pub name: String,
    pub node_fields: Vec<DataField>,
}

impl DataStruct {
    pub fn new(toks: Vec<AdvancedToken>) -> Result<DataStruct, AppErr> {
        // extracting data type name
        let second_tok_opt = toks.get(1).clone();
        if second_tok_opt.is_none() {
            return Err(app_err!(AppErrKind::Parser(ParserErr::MalformedDataType(
                String::from(
                    "attempted to access token containing data type name but could not locate it"
                )
            ))));
        }
        let second_tok = second_tok_opt.unwrap();
        let data_type_name: String;
        if let AdvancedToken::Indicator(s) = second_tok {
            data_type_name = s.clone();
        } else {
            return Err(app_err!(AppErrKind::Parser(
                ParserErr::ExpectedIndicatorToken(String::from(
                    "attempted to access tokens for ast generation, and expected an indicator token, but could not find one"
                ))
            )));
        }

        let mut field_names: Vec<String> = vec![];
        let mut field_types: Vec<DataType> = vec![];
        

        // 05/22/26 - was using even odd approach for field type/value extraction, had to implement new approach with addition or tracking '[]' tokens –– opted in to just use an generic lexer for the job because it includes peeking for easier extraction
        
        let mut found_name = false;
        let field_toks: Vec<AdvancedToken> = toks.into_iter().filter_map(|t| {
            match t {
                AdvancedToken::Indicator(_) => { 
                    if !found_name {
                        found_name = true;
                        return None
                    }
                    Some(t)
                },
                AdvancedToken::KeywordNum => { Some(t) },
                AdvancedToken::KeywordBool => { Some(t) },
                AdvancedToken::KeywordStr => { Some(t) },
                AdvancedToken::ArrayIndication => { Some(t) }
                _ => { None }
            }
        }).collect();

        let mut tok_lex = lexer::Lexer::new(field_toks);

        while !tok_lex.at_end() {

            let is_array = match tok_lex.peek(2) {
                AdvancedToken::ArrayIndication => true,
                _ => false
            };

            match tok_lex.item() {
                AdvancedToken::Indicator(s) => {
                    field_names.push(s);
                    match tok_lex.peek(1) {
                        AdvancedToken::KeywordBool => {
                            if is_array {
                                field_types.push(DataType::Array(Box::new(DataType::Bool)))
                            } else {
                                field_types.push(DataType::Bool)
                            }
                        },
                        AdvancedToken::KeywordStr => {
                            if is_array {
                                field_types.push(DataType::Array(Box::new(DataType::Str))) 
                            } else {
                                field_types.push(DataType::Str)
                            }
                        },
                        AdvancedToken::KeywordNum => {
                            if is_array {
                                field_types.push(DataType::Array(Box::new(DataType::Num))) 
                            } else {
                                field_types.push(DataType::Num)
                            }
                        },
                        AdvancedToken::Indicator(s) => {
                            if is_array {
                                field_types.push(DataType::Array(Box::new(DataType::Custom(s))))
                            } else {
                                field_types.push(DataType::Custom(s))
                            }
                        },
                        AdvancedToken::ClosedCurlyBrace => {

                        },
                        AdvancedToken::SemiColon => {

                        },
                        _ => {
                            return Err(app_err!(AppErrKind::Parser(ParserErr::MalformedStruct(format!("struct named {} is malformed", data_type_name)))))
                        }
                    }
                },
                _ => {}
            }

            if is_array {
                tok_lex.next();
                tok_lex.next();
                tok_lex.next(); 
            } else {
                tok_lex.next();
                tok_lex.next();
            }

        
        }

        // our field names and data types should be same len
        if field_names.len() != field_types.len() {
            return Err(app_err!(AppErrKind::Parser(ParserErr::MalformedDataType(
                String::from(
                    "expected our field names and field types to be the same length but they were not"
                )
            ))));
        }

        let mut fields: Vec<DataField> = vec![];
        let mut count: usize = 0;
        loop {
            if count > field_names.len() - 1 || count > field_types.len() - 1 {
                break;
            }
            let name_opt = field_names.get(count);
            if name_opt.is_none() {
                return Err(app_err!(AppErrKind::Parser(ParserErr::MalformedDataType(
                    String::from("could not find field name in expected location")
                ))));
            }
            let name = name_opt.unwrap();
            let t_opt = field_types.get(count);
            if t_opt.is_none() {
                return Err(app_err!(AppErrKind::Parser(ParserErr::MalformedDataType(
                    String::from("could not find field type in expected location")
                ))));
            }
            let t = t_opt.unwrap();
            let field = DataField::new(name.to_owned(), t.clone());
            fields.push(field);
            count = count + 1
        }
        return Ok(DataStruct {
            name: data_type_name,
            node_fields: fields,
        });
    }

    pub fn get_substruct_names(
        &self,
        mut vec: Vec<String>,
        data_struct_map: &HashMap<String, DataStruct>,
        parent: &DataStruct
    ) -> Result<Vec<String>, AppErr> {

        let nodes = self.node_fields.clone();

        for field in nodes.into_iter() {
            match field.data_type {
                DataType::Str => {},
                DataType::Num => {},
                DataType::Bool => {},
                DataType::Custom(substruct_name) => {
                    let inner_struct = match data_struct_map.get(&substruct_name) {
                        Some(data_struct) => data_struct,
                        None => {
                            return Err(err_invalid_struct_access(substruct_name.clone()));
                        }
                    };
                    vec = inner_struct.get_substruct_names(vec, data_struct_map, &parent)?;
                        let inner_struct_name = inner_struct.name.clone();
                        if vec.contains(&inner_struct_name) {
                            return Err(app_err!(AppErrKind::Parser(ParserErr::InfiniteStructReference(format!("the struct named {} has an infinite reference loop", parent.name)))))
                        } 
                    vec.push(inner_struct.name.clone());
                }
                DataType::Array(data_type_box) => {
                    let inner_data_type = *data_type_box;
                    match &inner_data_type {
                        DataType::Str => {},
                        DataType::Num => {},
                        DataType::Bool => {},
                        DataType::Custom(s) => {
                            let array_data_type_name = s.as_str().to_string();
                            let inner_struct = match data_struct_map.get(&array_data_type_name) {
                                Some(data_struct) => data_struct,
                                None => {
                                    return Err(err_invalid_struct_access(array_data_type_name.clone()));
                                }
                            };

                            vec = inner_struct.get_substruct_names(vec, data_struct_map, &parent)?;
                            let inner_struct_name = inner_struct.name.clone();
                            if vec.contains(&inner_struct_name) {
                                return Err(app_err!(AppErrKind::Parser(ParserErr::InfiniteStructReference(format!("the struct named {} has an infinity reference loop", parent.name)))))
                            }
                            vec.push(inner_struct.name.clone());
                        },
                        DataType::Array(data_type) => {}, // <== deeply nested arrays?
                    };
                },
            }
        }
        return Ok(vec);
    }
}
