use crate::err::ErrMsg;

#[derive(Debug, thiserror::Error)]
pub enum ErrInterpreter {

    #[error("{0}")]
    StructDuplication(ErrMsg),

    #[error("{0}")]
    VariableDuplication(ErrMsg),

    #[error("{0}")]
    InvalidVariableType(ErrMsg),

    #[error("{0}")]
    InvalidStructAccess(ErrMsg),

}