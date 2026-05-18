use crate::{cli::args::ArgsErr, err::AppErr};




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