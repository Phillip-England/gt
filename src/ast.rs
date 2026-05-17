use crate::{err::AppErr, lexer::GenericLex, tokenizer::Token};



#[derive(Debug, Clone)]
pub enum AstNode {
    DataType(String),
}

#[derive(Debug, Clone)]
pub struct Ast {
    head: AstNode
}

pub enum AstErr {
    MissingClosingCurlyBrace(String),
    MissingOpeningCurlyBrace(String),
    ExpectedIndicatorToken(String),
    MalformedDataType(String),
}


pub fn handle_ast_err(err: AstErr) {
    match err {
        AstErr::MissingClosingCurlyBrace(s) => {
            eprintln!("missing closing curly brace: {}", s);
        },
        AstErr::MissingOpeningCurlyBrace(s) => {
            eprintln!("missing opening curly brace: {}", s);
        },
        AstErr::ExpectedIndicatorToken(s) => {
            eprintln!("expected indicator token but did not find one: {}", s);
        },
        AstErr::MalformedDataType(s) => {
            eprintln!("unexpected data type structure encountered: {}", s);
        }
    }
}

pub fn new_ast(toks: Vec<Token>) -> Result<Ast, AppErr> {
    
    let head: AstNode;


    let mut l: GenericLex<Token> = GenericLex::new(toks);
    loop {
        let tok = l.item();
        match tok {
            Token::Colon => {

            },
            Token::Indicator(s) => {

            },
            Token::KeywordData => {
                let node = handle_data_token(&mut l)?;
                println!("{:?}", node);
            },
            Token::KeywordNum => {

            },
            Token::KeywordStr => {

            },
            Token::KeywordBool => {

            },
            Token::OperatorAssignment => {

            },
            Token::PromptEnd => {

            },
            Token::PromptName(s) => {

            },
            Token::PromptStart => {

            },
            Token::PromptText(s) => {

            },
            Token::SymbolClosedCurlyBrace => {

            },
            Token::SymbolOpenedCurlyBrace => {

            }

        }

        if l.at_end() {
            break;
        }
        l.next();
    }

    head = AstNode::DataType(String::from("yo"));
    let ast: Ast = Ast {
        head: head,
    };

    return Ok(ast);
}


pub fn new_ast_node(tok: Token) -> Result<(), ()> {
    return Ok(())
}


pub fn handle_data_token(l: &mut GenericLex<Token>) -> Result<NodeDataType, AppErr> {
    let mut is_data_keyword = false;
    if matches!(l.peek(1), Token::Indicator(s)) {
        if matches!(l.peek(2), Token::SymbolOpenedCurlyBrace) {
            if matches!(l.peek(3), Token::Indicator(s2)) {
                is_data_keyword = true;
            }
        }
    }
    if !is_data_keyword {
        return Err(AppErr::Ast(AstErr::MissingOpeningCurlyBrace(String::from("expected to find opening curly brace after data keyword but failed to find it")))) 
    }
    l.mark();
    loop {
        if matches!(l.item(), Token::SymbolClosedCurlyBrace) {
            break;
        }
        if l.at_end() {
            return Err(AppErr::Ast(AstErr::MissingClosingCurlyBrace(String::from("expected to find a closing curly brace for our data keyword but failed to find it")))) 
        }
        l.next();
    }
    l.next();
    let ast_toks = l.collect(l.marked_pos, l.pos);
    
    // extracting data type name
    let second_tok_opt = ast_toks.get(1).clone();
    if second_tok_opt.is_none() {
        return Err(AppErr::Ast(AstErr::MalformedDataType(String::from("attempted to access token containing data type name but could not locate it"))))
    }
    let second_tok = second_tok_opt.unwrap();
    let mut data_type_name: String;
    if let Token::Indicator(s) = second_tok {
        data_type_name = s.clone();
    } else {
        return Err(AppErr::Ast(AstErr::ExpectedIndicatorToken(String::from("attempted to access tokens for ast generation, and expected an indicator token, but could not find one"))))   
    }


    let mut count = 0;
    let mut field_names: Vec<String> = vec![];
    let mut field_types: Vec<DataType> = vec![];
    for tok in ast_toks {
        if count > 2 {
            if matches!(tok, Token::SymbolClosedCurlyBrace) {
                count = count + 1;
                continue
            }
            // odd should be field name
            if count % 2 == 1 {
                if matches!(tok, Token::Indicator(s)) {
                    if let Token::Indicator(s) = tok {
                        let field_name = s.clone();
                        field_names.push(field_name);
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

    let mut fields: Vec<NodeDataTypeField> = vec![];
    let mut count: usize = 0;
    println!("{:?} {:?}", field_names, field_types);
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
        let field = NodeDataTypeField::new(name.to_owned(), t.clone());
        fields.push(field);
        count = count + 1
    }
    let node_data_type = NodeDataType::new(data_type_name, fields);
    return Ok(node_data_type)
}

#[derive(Clone, Debug)]
pub struct NodeDataType {
    name: String,
    node_fields: Vec<NodeDataTypeField>
}

impl NodeDataType {


    pub fn new(name: String, node_fields: Vec<NodeDataTypeField>) -> NodeDataType {
        return NodeDataType {
            name,
            node_fields,
        }
    }

}

#[derive(Clone, Debug)]
pub struct NodeDataTypeField {
    name: String,
    t: DataType
}

impl NodeDataTypeField {

    pub fn new(name: String, t: DataType) -> NodeDataTypeField {
        return NodeDataTypeField {
            name,
            t,
        }
    }

}


#[derive(Clone, Debug)]
pub enum DataType {
    Str,
    Num,
    Bool,
}