use crate::err::ErrMsg;

#[derive(Debug, thiserror::Error)]
pub enum ErrParser {

    #[error("{0}")]
    MissingOpeningCurlyBrace(ErrMsg),

    #[error("{0}")]
    ExpectedIndicatorToken(ErrMsg),

    #[error("{0}")]
    MalformedDataStruct(ErrMsg),

    #[error("{0}")]
    MalformedVariable(ErrMsg),

    #[error("{0}")]
    MissingSemiColon(ErrMsg),

    #[error("{0}")]
    MissingArrayIndication(ErrMsg),

    #[error("{0}")]
    InfiniteStructReference(ErrMsg),

    #[error("{0}")]
    InvalidStructAccess(ErrMsg),

}