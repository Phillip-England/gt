use std::{env};

use crate::err::AppErr; 

#[derive(Debug)]
pub enum ArgsErr {
    MissingArgs,
    ArgDoesNotExist(usize)
}

pub fn handle_arg_err(err: ArgsErr) {
    match err {
        ArgsErr::MissingArgs => {
            eprintln!("missing args, please run 'gt help' for available commands");
        },
        ArgsErr::ArgDoesNotExist(pos) => {
            eprintln!("arg does not exist at position: {}", pos);
        }
    }
}

pub fn load_args() -> Result<Vec<String>, AppErr> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(AppErr::Args(ArgsErr::MissingArgs))
    }
    return Ok(args);
}

pub fn get_arg(args: Vec<String>, pos: usize) -> Result<(Vec<String>, String), AppErr> {
    let arg_opt = args.get(pos);
    let final_arg: String;
    match arg_opt {
        Some(arg) => {
            final_arg = arg.to_string();
        },
        None => {
          return Err(AppErr::Args(ArgsErr::ArgDoesNotExist(pos))); 
        }
    }
    return Ok((args, final_arg))
} 