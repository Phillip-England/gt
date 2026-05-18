
use crate::{err, lexer::GenericLex};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    KeywordData,
    KeywordLet,
    KeywordStr,
    KeywordNum,
    KeywordBool,
    Colon,
    SemiColon,
    OperatorAssignment,
    ClosedCurlyBrace,
    OpenedCurlyBrace,
    VariableName(String),
    Indicator(String),
    PromptStart,
    PromptEnd,
    PromptText(String),
    EndOfFile

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
                if s == "bool" {
                    sorted.push(Token::KeywordBool);
                    continue;
                }
                if s == "=" {
                    sorted.push(Token::OperatorAssignment);
                    continue
                }
                if s == "{" {
                    sorted.push(Token::OpenedCurlyBrace);
                    continue;
                }
                if s == "};" {
                    sorted.push(Token::ClosedCurlyBrace);
                    sorted.push(Token::SemiColon);
                    continue;
                }
                if s == "}" {
                    sorted.push(Token::ClosedCurlyBrace);
                    continue;
                }
                if s.ends_with(":") {
                    s.pop();
                    sorted.push(Token::VariableName(s));
                    sorted.push(Token::Colon);
                    continue;
                }
                if s == "let" {
                    sorted.push(Token::KeywordLet);
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
            },
            LexerToken::SemiColon => {
                sorted.push(Token::SemiColon)
            }
        }
    }
    sorted.push(Token::EndOfFile);
    Ok(sorted)
}

pub fn derive_basic_tokens(content: String) -> Result<Vec<LexerToken>, err::AppErr> {
    let chars: Vec<char> = content.chars().collect();
    let mut toks: Vec<LexerToken> = vec![];
    let mut l: GenericLex<char> = GenericLex::new(chars);
    let mut state = State::Init;
    loop {
        match state {

            State::Init => {
                if l.item() == ' ' || l.item() == '\n' {
                    l.next();
                    if l.at_end() {
                        break;
                    }
                    continue;
                }
                state = State::AtWordStart;
            },

            State::AtWordStart => {
                if l.item() == ' ' || l.item() == '\n' {
                    state = State::Init;
                    continue;
                }
                l.mark();
                loop {
                    if l.item() == ' ' || l.item() == '\n' || l.at_end() {
                        break;
                    }
                    l.next();
                }
                let word: String = l.collect(l.marked_pos(), l.pos()).into_iter().collect();

                if word == "<?" {
                    toks.push(LexerToken::PromptStart);
                    l.mark();
                    loop {

                        if l.item() != '>' {
                            l.next();
                            if l.at_end() {
                                break;
                            }
                            continue;
                        }
                        if l.at_end() {
                            break;
                        }
                        if l.peek(-1) != '?' || l.peek(1) != ';' {
                            l.next();
                            if l.at_end() {
                                break;
                            }
                            continue;
                        }
                        let mut prompt_str: String = l.collect(l.marked_pos(), l.pos()).into_iter().collect();
                        prompt_str.pop();
                        prompt_str.pop();
                        toks.push(LexerToken::PromptText(prompt_str.trim().to_string()));
                        toks.push(LexerToken::PromptEnd);
                        toks.push(LexerToken::SemiColon);
                        l.next();
                        l.next();
                        break;
                    }
                    continue;
                }

                toks.push(LexerToken::Indicator(word));

                l.next();
                if l.at_end() {
                    break;
                }

            }


        } 
        
    }



    return Ok(toks)   
}



pub fn tokenize(s: String) -> Result<Vec<Token>, err::AppErr> {
    let basic_tokens = derive_basic_tokens(s)?;
    println!("{:?}", basic_tokens);
    let refined_tokens = refine_tokens(basic_tokens)?;
    return Ok(refined_tokens);
}


#[derive(Debug, PartialEq, Eq, Clone)]
pub enum LexerToken {
    Indicator(String),
    PromptEnd,
    PromptStart,
    PromptText(String),
    SemiColon
}


#[derive(Debug, PartialEq, Eq)]
enum State {
    Init,
    AtWordStart,
}