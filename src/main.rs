use crate::{cli::cmd::{PrimaryCmd, primary_cmd_as_str}, err::{AppErr, handle_app_err}}; 

mod compiler;
mod cli;
mod err;
mod io;
mod interpreter;
mod llm;

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

    let args = cli::args::load_args()?; 
    let (args, first_arg) = cli::args::get_arg(args, 1)?; 

    if first_arg == primary_cmd_as_str(PrimaryCmd::Help) {
        cli::cmd::help();
        return Ok(());
    }

    if first_arg == primary_cmd_as_str(PrimaryCmd::Run)  {
        let (_args, filepath) = cli::args::get_arg(args, 2)?;
        cli::cmd::run(filepath)?;
        return Ok(())
    }

    eprintln!("no valid args provided, please run 'gt help' to see available commands");

    return Ok(())
}

