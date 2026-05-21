use std::{
    io::{BufRead, BufReader},
    process::Command,
};

use reqwest::blocking::{Client, Response};
use serde::Deserialize;
use serde_json::json;

use crate::{
    app_err,
    err::{AppErr, AppErrKind},
};

pub const OLLAMA_ADDR: &str = "http://localhost:11434";

pub fn is_ollama_installed() -> bool {
    Command::new("ollama").arg("--version").output().is_ok()
}

pub fn err_if_no_ollama() -> Result<(), AppErr> {
    if !is_ollama_installed() {
        return Err(app_err!(AppErrKind::Llm(LlmErr::OllamaNotInstalled)));
    }
    Ok(())
}

pub fn get_installed_ollama_models() -> Result<Vec<String>, AppErr> {
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
            return Err(app_err!(AppErrKind::Llm(LlmErr::UnexpectedCliErr(
                format!("{:?}", err)
            ))));
        }
    }
    return Ok(model_names);
}

#[derive(Debug)]
pub enum LlmErr {
    OllamaNotInstalled,
    UnexpectedCliErr(String),
    HttpRequestFailure(String),
}

pub fn handle_err(err: LlmErr) {
    match err {
        LlmErr::OllamaNotInstalled => {
            eprintln!("ollama is not installed and is required")
        }
        LlmErr::UnexpectedCliErr(s) => {
            eprintln!("an unexpected cli error occured: {}", s);
        }
        LlmErr::HttpRequestFailure(s) => {
            eprintln!("a request to the llm has failed: {}", s);
        }
    }
}

pub fn stream_prompt(client: &Client, model: &str, prompt: &str) -> Result<OllamaResponse, AppErr> {
    let result = client
        .post(OLLAMA_ADDR.to_string() + "/api/generate")
        .json(&json!({
            "model": model,
            "prompt": prompt,
            "stream": true
        }))
        .send();
    let response: Response;
    let mut err: Option<AppErr>;
    match result {
        Ok(r) => {
            response = r;
        }
        Err(e) => {
            return Err(app_err!(AppErrKind::Llm(LlmErr::HttpRequestFailure(
                format!("{:?}", e)
            ))));
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
                return Err(app_err!(AppErrKind::Llm(LlmErr::HttpRequestFailure(
                    format!("{:?}", e)
                ))));
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
