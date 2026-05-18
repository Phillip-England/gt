use crate::{err::AppErr, tokenizer::{AdvancedToken, BasicToken}};






pub fn get_advanced_tokens(toks: Vec<BasicToken>) -> Result<Vec<AdvancedToken>, AppErr> {

    let mut sorted: Vec<AdvancedToken> = vec![];
    for tok in toks {
        match tok {
            BasicToken::Indicator(mut s) => {
                if s == "data" {
                    sorted.push(AdvancedToken::KeywordData);
                    continue;
                }
                if s == "str" {
                    sorted.push(AdvancedToken::KeywordStr);
                    continue;
                }
                if s == "num" {
                    sorted.push(AdvancedToken::KeywordNum);
                    continue;
                }
                if s == "bool" {
                    sorted.push(AdvancedToken::KeywordBool);
                    continue;
                }
                if s == "=" {
                    sorted.push(AdvancedToken::OperatorAssignment);
                    continue
                }
                if s == "{" {
                    sorted.push(AdvancedToken::OpenedCurlyBrace);
                    continue;
                }
                if s == "};" {
                    sorted.push(AdvancedToken::ClosedCurlyBrace);
                    sorted.push(AdvancedToken::SemiColon);
                    continue;
                }
                if s == "}" {
                    sorted.push(AdvancedToken::ClosedCurlyBrace);
                    continue;
                }
                if s.ends_with(":") {
                    s.pop();
                    sorted.push(AdvancedToken::VariableName(s));
                    sorted.push(AdvancedToken::Colon);
                    continue;
                }
                if s == "let" {
                    sorted.push(AdvancedToken::KeywordLet);
                    continue;
                }
                // if we didnt find match
                sorted.push(AdvancedToken::Indicator(s));
            },
            BasicToken::PromptEnd => {
                sorted.push(AdvancedToken::PromptEnd);
            },
            BasicToken::PromptText(s) => {
                sorted.push(AdvancedToken::PromptText(s));
            },
            BasicToken::PromptStart => {
                sorted.push(AdvancedToken::PromptStart);
            },
            BasicToken::SemiColon => {
                sorted.push(AdvancedToken::SemiColon)
            }
        }
    }
    sorted.push(AdvancedToken::EndOfFile);
    Ok(sorted)
}