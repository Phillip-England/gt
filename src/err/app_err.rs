use crate::cli::args::ArgsErr;
use crate::compiler::parser::{self, ParserErr};
use crate::cli::{args};
use crate::file_manager::{self, FileManagerErr};




#[derive(Debug)]
pub enum AppErrKind {
    FileManager(FileManagerErr),
    Args(ArgsErr),
    Ast(ParserErr),
}


#[derive(Debug)]
pub struct AppErr {
    pub kind: AppErrKind,
    pub file: &'static str,
    pub line: u32,
}


#[macro_export]
macro_rules! app_err {
    ($kind:expr) => {
        AppErr {
            kind: $kind,
            file: file!(),
            line: line!(),
        }
    };
}


pub fn handle_app_err(app_err: AppErr) {
    eprintln!("ERROR: {:?}\nfile: {:?}\nline: {:?}", app_err.kind, app_err.file, app_err.line);
    match app_err.kind {
        AppErrKind::FileManager(compiler_err) => {
            file_manager::handle_file_manager_err(compiler_err);
        },
        AppErrKind::Args(args_err) => {
            args::handle_arg_err(args_err);
        },
        AppErrKind::Ast(ast_err) => {
            parser::handle_ast_err(ast_err);    
        }
    };
}