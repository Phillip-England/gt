use crate::{
    cli::cmd::{PrimaryCmd, primary_cmd_as_str},
    err::{ErrApp},
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
            println!("{:?}", err);
            std::process::exit(1);
        }
        _ => {}
    }
}

fn run() -> Result<(), ErrApp> {
    let args = cli::args::load_args()?;
    let (args, first_arg) = cli::args::get_arg(args, 1)?;


    /*  
        We do arg extraction at this level. No validation should need to be complete once we actually run our command. All of this should be done on this level so commands can solely focus on their implementation and none of the fluff
    */

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
