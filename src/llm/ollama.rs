use std::{
    io::{BufRead, BufReader},
    process::Command,
};

use reqwest::blocking::{Client, Response};
use serde::Deserialize;
use serde_json::json;

use crate::{err::{ErrApp, ErrMsg}, err_msg, fail};

pub const OLLAMA_ADDR: &str = "http://localhost:11434";

pub fn is_ollama_installed() -> bool {
    Command::new("ollama").arg("--version").output().is_ok()
}

pub fn err_if_no_ollama() -> Result<(), ErrLlm> {
    if !is_ollama_installed() {
        return Err(ErrLlm::OllamaNotInstalled(err_msg!("")).into());
    }
    Ok(())
}

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
            return fail!(ErrLlm::OllamaError, "ollama failed internally, here is the error: {}", err);
        }
    }
    return Ok(model_names);
}

#[derive(Debug, thiserror::Error)]
pub enum ErrLlm {

    #[error("{0}\nollama is not installed and is required")]
    OllamaNotInstalled(ErrMsg),

    #[error("{0}\nerror which originated from ollama cli during execution")]
    OllamaError(ErrMsg),

    #[error("{0} http request to ollama failed")]
    HttpRequestFailure(ErrMsg),

}


pub fn stream_prompt(client: &Client, model: &str, prompt: &str) -> Result<OllamaResponse, ErrLlm> {
    let result = client
        .post(OLLAMA_ADDR.to_string() + "/api/generate")
        .json(&json!({
            "model": model,
            "prompt": prompt,
            "stream": true
        }))
        .send();
    let response: Response;
    let mut err: Option<ErrLlm>;
    match result {
        Ok(r) => {
            response = r;
        }
        Err(e) => {
            return fail!(ErrLlm::HttpRequestFailure, "ollama http request failed, here is the internal ollama error: {}", e);
        }
    };

    let reader = BufReader::new(response);
    let mut col = "".to_string();
    let mut chunks: Vec<OllamaResponseChunk> = vec![];
    for line in reader.lines() {
        let mut trimmed_line: String;
        match line {
            Ok(line) => {
                if line.trim().is_empty() {
                    continue;
                }
                trimmed_line = line;
            }
            Err(e) => {
                return fail!(ErrLlm::HttpRequestFailure, "ollama http request failed, here is the internal ollama error: {}", e);
            }
        }
        let chunck: Result<OllamaResponseChunk, serde_json::Error> =
            serde_json::from_str(&trimmed_line);
        match chunck {
            Ok(chunck) => {
                col = col + &chunck.response;
                chunks.push(chunck);
            }
            Err(_) => {
                // unhandled
                // you may get a chunk which does not match the schema
                // in which case we just continue collecting
            }
        }
    }
    let ollama_response = OllamaResponse {
        model: model.to_string(),
        text: col,
        chunks: chunks,
    };
    return Ok(ollama_response);
}

#[derive(Debug, Deserialize)]
pub struct OllamaResponseChunk {
    pub model: String,
    pub created_at: String,
    pub response: String,
    pub done: bool,
}

#[derive(Debug, Deserialize)]
pub struct OllamaResponse {
    pub model: String,
    pub text: String,
    pub chunks: Vec<OllamaResponseChunk>,
}
