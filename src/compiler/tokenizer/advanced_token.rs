



#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AdvancedToken {
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
