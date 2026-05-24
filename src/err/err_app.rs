use crate::cli::args::ErrArgs;
use crate::compiler::parser::ErrParser;
use crate::compiler::validator::ErrValidator;
use crate::interpreter::ErrInterpreter;
use crate::io::ErrIo;
use crate::llm::ErrLlm;

#[derive(Debug, thiserror::Error)]
pub enum ErrApp {
    #[error(transparent)]
    Parser(#[from] ErrParser),

    #[error(transparent)]
    Args(#[from] ErrArgs),

    #[error(transparent)]
    Io(#[from] ErrIo),

    #[error(transparent)]
    Interpreter(#[from] ErrInterpreter),

    #[error(transparent)]
    Llm(#[from] ErrLlm),

    #[error(transparent)]
    Validator(#[from] ErrValidator)

}

#[derive(Debug, Clone)]
pub struct Loc {
    pub file: &'static str,
    pub line: u32,
    pub column: u32,
}

impl std::fmt::Display for Loc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.file, self.line, self.column)
    }
}

#[macro_export]
macro_rules! loc {
    () => {
        $crate::err::Loc {
            file: file!(),
            line: line!(),
            column: column!(),
        }
    };
}

#[derive(Debug, Clone)]
pub struct ErrMsg {
    pub loc: Loc,
    pub msg: String,
}

impl std::fmt::Display for ErrMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}", self.loc, self.msg)
    }
}

#[macro_export]
macro_rules! err_msg {
    ($($arg:tt)*) => {
        $crate::err::ErrMsg {
            loc: $crate::loc!(),
            msg: format!($($arg)*),
        }
    };
}

/*
DEBUGGING NOTE:

You will find youself here a lot dealing with the 'into()' call at the end of the macro below.

This may occur if you forget to return fail!() from your functions.

So, if you find yourself here, and the 'into()' call is not working, go investigate and make sure you are actually returning fail!() from all your functions. If you just call fail!() without returning, here you shall be.

Also, why are broke people so stubborn? Cause they got no change! Ha!

*/
#[macro_export]
macro_rules! fail {
    ($err:path, $($arg:tt)*) => {
        Err($err($crate::err_msg!($($arg)*)).into())
    };
}
