use crate::err::{ErrMsg, Loc};

#[derive(Debug, thiserror::Error)]
pub enum ErrIo {
    #[error("{0}\nfailed to load file")]
    ReadFileErr(ErrMsg),
}
