use crate::{
    app_err,
    cli::args::ArgsErr,
    err::{AppErr, AppErrKind},
};

pub fn get_arg(args: Vec<String>, pos: usize) -> Result<(Vec<String>, String), AppErr> {
    let arg_opt = args.get(pos);
    let final_arg: String;
    match arg_opt {
        Some(arg) => {
            final_arg = arg.to_string();
        }
        None => {
            return Err(app_err!(AppErrKind::Args(ArgsErr::ArgDoesNotExist(pos))));
        }
    }
    return Ok((args, final_arg));
}
