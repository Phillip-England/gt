use crate::{
    cli::cmd::{PrimaryCmd, primary_cmd_as_str},
    err::{AppErr, handle_app_err},
};

mod cli;
mod compiler;
mod err;
mod interpreter;
mod io;
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

    if first_arg == primary_cmd_as_str(PrimaryCmd::Run) {
        let (args, filepath) = cli::args::get_arg(args, 3)?;
        let (_args, model) = cli::args::get_arg(args, 2)?;
        cli::cmd::run(filepath, model)?;
        return Ok(());
    }

    eprintln!("no valid args provided, please run 'gt help' to see available commands");

    return Ok(());
}
