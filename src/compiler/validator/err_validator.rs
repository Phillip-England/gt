use crate::err::ErrMsg;



#[derive(Debug, thiserror::Error)]
pub enum ErrValidator {
    
    #[error("{0}\nWe encountered a dupliated data struct")]
    DuplicatedDataStruct(ErrMsg),

    #[error("{0}\nwe encountered a duplicated variable")]
    DuplicatedVariable(ErrMsg),

    #[error("{0}\ninvalid variable type")]
    InvalidVariableType(ErrMsg),

    #[error("{0}\n")]
    InvalidStructAccess(ErrMsg),

}


