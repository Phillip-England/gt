use std::env;

use crate::{
    cli::args::ErrArgs, fail,
};

pub fn load_args() -> Result<Vec<String>, ErrArgs> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return fail!(ErrArgs::VoidPrimaryArg, "");
    }
    return Ok(args);
}
