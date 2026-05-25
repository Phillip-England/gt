use std::{collections::HashMap}; 

use crate::{
    compiler::{
        lexer::Lexer,
        node::{DataStruct, Node, Variable},
        parser::{DataType, ErrParser, parse_data_struct, parse_variable},
        tokenizer::AdvancedToken, validator::ErrValidator,
    },
    err::ErrApp, fail, interpreter::ErrInterpreter, llm::Prompt,
};

#[derive(Debug, Clone)]
pub struct Ast {
    pub head_nodes: Vec<Node>,
    pub map_structs: HashMap<String, DataStruct>,
    pub map_vars: HashMap<String, Variable>,
    pub vec_prompts: Vec<Prompt>,
}

impl Ast {
 
    pub fn new(toks: Vec<AdvancedToken>) -> Result<Ast, ErrApp> {
        let mut ast = Ast { 
            head_nodes: vec![],
            map_structs: HashMap::new(),
            map_vars: HashMap::new(),
            vec_prompts: Vec::new(),
        };
        ast.extract_head_nodes(toks)?;
        ast.load_ast_hashmaps()?;
        ast.load_prompts()?;
        return Ok(ast);
    }

    pub fn extract_head_nodes(&mut self, toks: Vec<AdvancedToken>) -> Result<(), ErrParser> {
        let mut l: Lexer<AdvancedToken> = Lexer::new(toks);
        loop {
            let tok = l.item();
            match tok {
                AdvancedToken::ArrayIndication => {}
                AdvancedToken::Colon => {}
                AdvancedToken::SemiColon => {}
                AdvancedToken::Indicator(_) => {}
                AdvancedToken::EndOfFile => {}
                AdvancedToken::KeywordLet => {
                    let node_variable = parse_variable(&mut l)?;
                    self.head_nodes.push(Node::Variable(node_variable));
                }
                AdvancedToken::KeywordStruct => {
                    let node_data_struct = parse_data_struct(&mut l)?;
                    self.head_nodes.push(Node::DataStruct(node_data_struct));
                }
                AdvancedToken::KeywordNum => {}
                AdvancedToken::KeywordStr => {}
                AdvancedToken::KeywordBool => {}
                AdvancedToken::OperatorAssignment => {}
                AdvancedToken::PromptEnd => {}
                AdvancedToken::VariableName(_) => {}
                AdvancedToken::PromptStart => {}
                AdvancedToken::PromptValue(_) => {}
                AdvancedToken::ClosedCurlyBrace => {}
                AdvancedToken::OpenedCurlyBrace => {}
                AdvancedToken::DoubleQuote => {}
                AdvancedToken::StrValue(s) => {}
            }

            if l.at_end() {
                break;
            }
            l.next();
        }
        Ok(())
    }

    pub fn load_ast_hashmaps(&mut self) -> Result<(), ErrApp> {
        for node in self.head_nodes.drain(..) {
            match node {
                Node::DataStruct(data_struct) => {
                    let key = data_struct.name.clone();
                    let exists = self.map_structs.contains_key(&key);

                    if exists {
                        return fail!(
                            ErrValidator::DuplicatedDataStruct,
                            "the struct named {} is duplicated",
                            data_struct.name
                        );
                    }

                    self.map_structs.insert(data_struct.name.clone(), data_struct);
                }

                Node::Variable(variable) => {
                    let key = variable.name.clone();
                    let exists = self.map_vars.contains_key(&key);

                    if exists {
                        return fail!(
                            ErrValidator::DuplicatedVariable,
                            "the variable named {} has been duplicated",
                            variable.name
                        );
                    }

                    self.map_vars.insert(variable.name.clone(), variable);
                }
            }
        }
        return Ok(());
    }

    pub fn load_prompts(&mut self) -> Result<(), ErrApp> {
        let prompts: Vec<Prompt> = self.map_vars
            .iter()            
            .filter_map(|(_, variable)| match variable.data_type {
                DataType::Custom(_) => {
                    let prompt_toks: Vec<&AdvancedToken> = variable
                        .toks
                        .iter()
                        .filter(|tok| matches!(tok, AdvancedToken::PromptValue(_)))
                        .collect();

                    if prompt_toks.is_empty() {
                        None
                    } else {
                        return Some(Prompt::new(variable.name.clone(), variable.value.clone()));
                    }
                }

                _ => None,
            })
            .collect();
        self.vec_prompts = prompts;
        Ok(())
    }   

}
