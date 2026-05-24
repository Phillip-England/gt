use std::{
    io::{BufRead, BufReader},
    process::Command,
};

use reqwest::blocking::{Client, Response};
use serde_json::json;

use crate::{
    err::{ErrApp, ErrMsg},
    err_msg, fail,
    llm::{ErrLlm, OllamaResponse, OllamaResponseChunk},
};

pub const OLLAMA_ADDR: &str = "http://localhost:11434";

pub fn is_ollama_installed() -> bool {
    Command::new("ollama").arg("--version").output().is_ok()
}

pub fn err_if_no_ollama() -> Result<(), ErrLlm> {
    if !is_ollama_installed() {
        return fail!(ErrLlm::OllamaNotInstalled, "");
    }
    Ok(())
}


pub fn stream_prompt<F>(client: &Client, model: &str, prompt: &str, mut on_chunk: F) -> Result<OllamaResponse, ErrLlm> 
where 
    F: FnMut(&OllamaResponseChunk),
{
    let result = client
        .post(OLLAMA_ADDR.to_string() + "/api/generate")
        .json(&json!({
            "model": model,
            "prompt": prompt,
            "stream": true
        }))
        .send();
    let response: Response;
    match result {
        Ok(r) => {
            response = r;
        }
        Err(e) => {
            return fail!(
                ErrLlm::HttpRequestFailure,
                "ollama http request failed, here is the internal ollama error: {}",
                e
            );
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
                return fail!(
                    ErrLlm::HttpRequestFailure,
                    "ollama http request failed, here is the internal ollama error: {}",
                    e
                );
            }
        }
        let chunck: Result<OllamaResponseChunk, serde_json::Error> =
            serde_json::from_str(&trimmed_line);
        match chunck {
            Ok(chunck) => {
                on_chunk(&chunck);
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