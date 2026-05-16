use rlex::Rlex;

use crate::{err};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    KeywordData,
    KeywordStr,
    KeywordNum,
    Colon,
    OperatorAssignment,
    SymbolClosedCurlyBrace,
    SymbolOpenedCurlyBrace,
    PromptName(String),
    Indicator(String),
    PromptStart,
    PromptEnd,
    PromptText(String),

}



pub fn refine_tokens(toks: Vec<LexerToken>) -> Result<Vec<Token>, err::AppErr> {
    let mut sorted: Vec<Token> = vec![];
    for tok in toks {
        match tok {
            LexerToken::Indicator(mut s) => {
                if s == "data" {
                    sorted.push(Token::KeywordData);
                    continue;
                }
                if s == "str" {
                    sorted.push(Token::KeywordStr);
                    continue;
                }
                if s == "num" {
                    sorted.push(Token::KeywordNum);
                    continue;
                }
                if s == "=" {
                    sorted.push(Token::OperatorAssignment);
                    continue
                }
                if s == "{" {
                    sorted.push(Token::SymbolOpenedCurlyBrace);
                    continue;
                }
                if s == "}" {
                    sorted.push(Token::SymbolClosedCurlyBrace);
                    continue;
                }
                if s.ends_with(":") {
                    s.pop();
                    sorted.push(Token::PromptName(s));
                    sorted.push(Token::Colon);
                    continue;
                }
                // if we didnt find match
                sorted.push(Token::Indicator(s));
            },
            LexerToken::PromptEnd => {
                sorted.push(Token::PromptEnd);
            },
            LexerToken::PromptText(s) => {
                sorted.push(Token::PromptText(s));
            },
            LexerToken::PromptStart => {
                sorted.push(Token::PromptStart);
            }
        }
    }
    Ok(sorted)
}

pub fn derive_basic_tokens(content: String) -> Result<Vec<LexerToken>, err::AppErr> {
    let mut r: Rlex<LexerState, LexerToken> = Rlex::new(&content, LexerState::Init);
    
    loop {
        if r.at_end() {
            break;
        }
        match r.state() {
            &LexerState::Init => {
                if r.char() == ' ' || r.char() == '\n' {
                    r.next();
                    continue;
                }
                r.state_set(LexerState::AtWordStart);
            },
            &LexerState::AtWordStart => {
                if r.char() == ' ' || r.char() == '\n' {
                    r.state_set(LexerState::Init);
                    continue;
                }
                loop {
                    if r.at_end() || r.char() == ' ' || r.char() == '\n' {
                        break;
                    }
                    r.collect();
                    r.next();
                }
                let col = r.str_from_collection().to_string();
                r.collect_clear();

                if col == "<?" {
                    r.token_push(LexerToken::PromptStart);
                    r.mark();
                    loop {
                        if r.at_end() {
                            break;
                        }
                        r.next_until('>');
                        if r.peek_back() != '?' {
                            continue;
                        }
                        let mut prompt_str = r.str_from_mark().to_string();
                        prompt_str.pop();
                        prompt_str.pop();
                        r.token_push(LexerToken::PromptText(prompt_str.trim().to_string()));
                        r.token_push(LexerToken::PromptEnd);
                        r.next();
                        break;
                    }
                    continue;
                }

                r.token_push(LexerToken::Indicator(col));

            } 
        }

    }


    let toks = r.toks().clone();

    return Ok(toks)   
}



pub fn tokenize(s: String) -> Result<Vec<Token>, err::AppErr> {
    let basic_tokens = derive_basic_tokens(s)?;
    let refined_tokens = refine_tokens(basic_tokens)?;
    return Ok(refined_tokens);
}


#[derive(Debug, PartialEq, Eq, Clone)]
pub enum LexerToken {
    Indicator(String),
    PromptEnd,
    PromptStart,
    PromptText(String)
}


#[derive(Debug, PartialEq, Eq)]
pub enum LexerState {
    Init,
    AtWordStart
}