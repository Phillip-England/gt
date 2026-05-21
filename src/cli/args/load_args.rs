use std::env;

use crate::{
    app_err,
    cli::args::ArgsErr,
    err::{AppErr, AppErrKind},
};

pub fn load_args() -> Result<Vec<String>, AppErr> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(app_err!(AppErrKind::Args(ArgsErr::MissingArgs)));
    }
    return Ok(args);
}
