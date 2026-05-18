use crate::{cmd::{PrimaryCmd, primary_cmd_as_str}, err::{AppErr, handle_app_err}}; 

// dir-level
mod ast;

// file-level
mod err;
mod tokenizer;
mod lexer;
mod args;
mod cmd;
mod file_manager;

fn main() {
    match run() {
        Err(err) => {
            handle_app_err(err);
            std::process::exit(1);
        }
        _ => {}
    }
}
fn run() -> Result<(), AppErr> {

    let args = args::load_args()?; 
    let (args, first_arg) = args::get_arg(args, 1)?; 

    if first_arg == primary_cmd_as_str(PrimaryCmd::Help) {
        cmd::help();
        return Ok(());
    }

    if first_arg == primary_cmd_as_str(PrimaryCmd::Run)  {
        let (_args, filepath) = args::get_arg(args, 2)?;
        cmd::run(filepath)?;
        return Ok(())
    }

    eprintln!("no valid args provided, please run 'gt help' to see available commands");

    return Ok(())
}

