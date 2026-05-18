use std::env;

use crate::{cli::args::ArgsErr, err::AppErr};






pub fn load_args() -> Result<Vec<String>, AppErr> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(AppErr::Args(ArgsErr::MissingArgs))
    }
    return Ok(args);
}