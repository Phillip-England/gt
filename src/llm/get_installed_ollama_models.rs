use std::process::Command;

use crate::{fail, llm::ErrLlm};

pub fn get_installed_ollama_models() -> Result<Vec<String>, ErrLlm> {
    let mut model_names: Vec<String> = vec![];
    let out = Command::new("ollama").arg("list").output();
    match out {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let lines = stdout.split("\n");
            let mut is_first_line = true;
            for line in lines {
                if is_first_line {
                    is_first_line = false;
                    continue;
                }
                let parts: Vec<&str> = line.split_whitespace().collect();
                let model_name_opt = parts.first();
                match model_name_opt {
                    Some(model_name) => model_names.push(model_name.to_string()),
                    None => {}
                };
            }
        }
        Err(err) => {
            return fail!(
                ErrLlm::OllamaError,
                "ollama failed internally, here is the error: {}",
                err
            );
        }
    }
    return Ok(model_names);
}
