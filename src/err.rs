use crate::args;
use crate::file_manager;



pub enum AppErr {
    FileManager(file_manager::FileManagerErr),
    Args(args::ArgsErr),
}


pub fn handle_app_err(app_err: AppErr) {
    match app_err {
        AppErr::FileManager(compiler_err) => {
            file_manager::handle_file_manager_err(compiler_err);
        },
        AppErr::Args(args_err) => {
            args::handle_arg_err(args_err);
        },
    };
}