use std::process::Command;

use crate::{app_err, err::{AppErr, AppErrKind}};

pub fn is_ollama_installed() -> bool {
    Command::new("ollama")
        .arg("--version")
        .output()
        .is_ok()
}


pub fn err_if_no_ollama() -> Result<(), AppErr> {
    if !is_ollama_installed() {
        return Err(app_err!(AppErrKind::Llm(LlmErr::OllamaNotInstalled)))
    }
    Ok(())
}

#[derive(Debug)]
pub enum LlmErr {
    OllamaNotInstalled,
}

pub fn handle_err(err: LlmErr) {
    match err {
        LlmErr::OllamaNotInstalled => {
            eprintln!("ollama is not installed and is required")
        }
    }
}