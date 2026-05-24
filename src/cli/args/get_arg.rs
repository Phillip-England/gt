use crate::{
    cli::args::ErrArgs, fail,
};

pub fn get_arg(args: Vec<String>, pos: usize) -> Result<(Vec<String>, String), ErrArgs> {
    let arg_opt = args.get(pos);
    let final_arg: String;
    match arg_opt {
        Some(arg) => {
            final_arg = arg.to_string();
        }
        None => {
            return fail!(ErrArgs::ArgDoesNotExist, "arg does not exist at this position: {}", pos);
        }
    }
    return Ok((args, final_arg));
}
