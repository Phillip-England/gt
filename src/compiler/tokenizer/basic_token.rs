#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BasicToken {
    Indicator(String),
    PromptEnd,
    PromptStart,
    PromptText(String),
    SemiColon,
    DoubleQuote,
    StrValue(String),
}
