use crate::err::Loc;




#[derive(Debug, thiserror::Error)]
pub enum ErrIo {

    #[error("{loc}\n failed to load file at path: {path}")]
    LoadFileErr {
        loc: Loc,
        path: String,
    },
}
