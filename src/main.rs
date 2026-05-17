use crate::err::{AppErr}; 

mod err;
mod ast;
mod tokenizer;
mod lexer;
mod args;
mod cmd;
mod file_manager;

fn main() {

    let args = match args::load_args() {
        Ok(args) => args,
        Err(err) => {
            let app_err = AppErr::Args(err);
            err::handle_app_err(app_err);
            return;
        }
    };

    let (args, first_arg) = match args::get_arg(args, 1) {
        Ok(arg) => arg,
        Err(err) => {
            args::handle_arg_err(err);
            return;
        }
    }; 

    if first_arg == "help" {
        cmd::help();
        return;
    }

    if first_arg == "run"  {
        let (_args, filepath) = match args::get_arg(args, 2) {
            Ok(arg) => arg,
            Err(err) => {
                args::handle_arg_err(err);
                return;
            }
        };
        let result = cmd::run(filepath);
        match result {
            Ok(_v) => {

            },
            Err(err) => {
                err::handle_app_err(err);
                return;
            }
        }
    }
    

    eprintln!("no valid args provided, please run 'gt help' to see available commands");

}
