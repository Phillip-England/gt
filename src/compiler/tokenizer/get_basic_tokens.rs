use crate::{
    compiler::lexer::Lexer,
    compiler::tokenizer::{BasicToken, BasicTokenState},
    err::AppErr,
};

pub fn get_basic_tokens(content: String) -> Result<Vec<BasicToken>, AppErr> {
    let chars: Vec<char> = content.chars().collect();
    let mut toks: Vec<BasicToken> = vec![];
    let mut l: Lexer<char> = Lexer::new(chars);
    let mut state = BasicTokenState::Init;
    loop {
        match state {
            BasicTokenState::Init => {
                if l.item() == ' ' || l.item() == '\n' {
                    l.next();
                    if l.at_end() {
                        break;
                    }
                    continue;
                }
                state = BasicTokenState::AtWordStart;
            }

            BasicTokenState::AtWordStart => {
                if l.item() == ' ' || l.item() == '\n' {
                    state = BasicTokenState::Init;
                    continue;
                }
                l.mark();
                loop {
                    if l.item() == ' ' || l.item() == '\n' || l.at_end() {
                        break;
                    }
                    l.next();
                }
                let mut word: String = l.collect(l.marked_pos(), l.pos()).into_iter().collect();

                if word == "<?" {
                    toks.push(BasicToken::PromptStart);
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
                        let mut prompt_str: String =
                            l.collect(l.marked_pos(), l.pos()).into_iter().collect();
                        prompt_str.pop();
                        prompt_str.pop();
                        toks.push(BasicToken::PromptText(prompt_str.trim().to_string()));
                        toks.push(BasicToken::PromptEnd);
                        toks.push(BasicToken::SemiColon);
                        l.next();
                        l.next();
                        break;
                    }
                    continue;
                }

                if word.starts_with("\"") && word.ends_with("\";") {
                    word.remove(0);
                    word.pop();
                    word.pop();
                    toks.push(BasicToken::DoubleQuote);
                    toks.push(BasicToken::StrValue(word.clone()));
                    toks.push(BasicToken::DoubleQuote);
                    toks.push(BasicToken::SemiColon);
                    l.next();
                    continue;
                }


                if word.ends_with("[]") {
                    word.pop();
                    word.pop();
                    toks.push(BasicToken::Indicator(word));
                    toks.push(BasicToken::ArrayIndication);
                    l.next();
                    continue
                }

                toks.push(BasicToken::Indicator(word));

                l.next();
                if l.at_end() {
                    break;
                }
            }
        }
    }

    return Ok(toks);
}
