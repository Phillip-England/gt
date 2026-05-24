use crate::err::{ErrMsg, Loc};



#[derive(Debug, thiserror::Error)]
pub enum ErrArgs {

    #[error("{0}\nyou attempted to run the application with an arg which does not exist, pleas run 'gt help' for more information")]
    VoidPrimaryArg(ErrMsg),

    #[error("{0}\narg does not exist")]
    ArgDoesNotExist(ErrMsg),


}