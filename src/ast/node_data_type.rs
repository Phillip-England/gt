use crate::{ast::{AstErr, NodeDataField, DataType}, err::AppErr, tokenizer::Token};



#[derive(Clone, Debug)]
pub struct NodeDataType {
    name: String,
    node_fields: Vec<NodeDataField>
}

impl NodeDataType {

    pub fn new(toks: Vec<Token>) -> Result<NodeDataType, AppErr> {
       
        // extracting data type name
        let second_tok_opt = toks.get(1).clone();
        if second_tok_opt.is_none() {
            return Err(AppErr::Ast(AstErr::MalformedDataType(String::from("attempted to access token containing data type name but could not locate it"))))
        }
        let second_tok = second_tok_opt.unwrap();
        let data_type_name: String;
        if let Token::Indicator(s) = second_tok {
            data_type_name = s.clone();
        } else {
            return Err(AppErr::Ast(AstErr::ExpectedIndicatorToken(String::from("attempted to access tokens for ast generation, and expected an indicator token, but could not find one"))))   
        }


        let mut count = 0;
        let mut field_names: Vec<String> = vec![];
        let mut field_types: Vec<DataType> = vec![];
        for tok in toks {
            if count > 2 {
                if matches!(tok, Token::ClosedCurlyBrace) {
                    count = count + 1;
                    continue
                }
                // odd should be field name
                if count % 2 == 1 {
                    if matches!(tok, Token::Indicator(_)) {
                        match tok {
                            Token::Indicator(s) => {
                                let field_name = s.to_owned();
                                field_names.push(field_name);
                            },
                            (_) => {}
                        }
                    }
                    count = count + 1;
                    continue
                }
                // even should be field type
                if matches!(tok, Token::KeywordNum) {
                    field_types.push(DataType::Num)
                }
                if matches!(tok, Token::KeywordStr) {
                    field_types.push(DataType::Str)
                }
                if matches!(tok, Token::KeywordBool) {
                    field_types.push(DataType::Bool)
                }

            }
            count = count + 1;
        }

        // our field names and data types should be same len
        if field_names.len() != field_types.len() {
            return Err(AppErr::Ast(AstErr::MalformedDataType(String::from("expected our field names and field types to be the same length but they were not"))));
        }

        let mut fields: Vec<NodeDataField> = vec![];
        let mut count: usize = 0;
        loop {
            if count > field_names.len() - 1 || count > field_types.len() - 1 {
                break;
            }
            let name_opt = field_names.get(count);
            if name_opt.is_none() {
                println!("{:?}", fields);
                return Err(AppErr::Ast(AstErr::MalformedDataType(String::from("could not find field name in expected location"))))
            }
            let name = name_opt.unwrap();
            let t_opt = field_types.get(count);
            if t_opt.is_none() {
                return Err(AppErr::Ast(AstErr::MalformedDataType(String::from("could not find field type in expected location"))))
            }        
            let t = t_opt.unwrap();
            let field = NodeDataField::new(name.to_owned(), t.clone());
            fields.push(field);
            count = count + 1
        }
        return Ok(NodeDataType {
            name: data_type_name,
            node_fields: fields,
        })
    }

}

