use crate::{
    compiler::tokenizer::{AdvancedToken, BasicToken},
    err::AppErr,
};

pub fn get_advanced_tokens(toks: Vec<BasicToken>) -> Result<Vec<AdvancedToken>, AppErr> {
    let mut sorted: Vec<AdvancedToken> = vec![];
    for tok in toks {
        match tok {
            BasicToken::ArrayIndication => {
                sorted.push(AdvancedToken::ArrayIndication);
                continue;
            },
            BasicToken::Indicator(mut s) => {
                if s == "struct" {
                    sorted.push(AdvancedToken::KeywordStruct);
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
                    continue;
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
            }
            BasicToken::PromptEnd => {
                sorted.push(AdvancedToken::PromptEnd);
            }
            BasicToken::PromptText(s) => {
                sorted.push(AdvancedToken::PromptValue(s));
            }
            BasicToken::PromptStart => {
                sorted.push(AdvancedToken::PromptStart);
            }
            BasicToken::SemiColon => {
                sorted.push(AdvancedToken::SemiColon);
            }
            BasicToken::DoubleQuote => {
                sorted.push(AdvancedToken::DoubleQuote);
            }
            BasicToken::StrValue(s) => {
                sorted.push(AdvancedToken::StrValue(s));
            }
        }
    }
    sorted.push(AdvancedToken::EndOfFile);
    Ok(sorted)
}
