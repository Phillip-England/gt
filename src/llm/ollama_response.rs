use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct OllamaResponse {
    pub model: String,
    pub text: String,
    pub chunks: Vec<OllamaResponseChunk>,
}

#[derive(Debug, Deserialize)]
pub struct OllamaResponseChunk {
    pub model: String,
    pub created_at: String,
    pub response: String,
    pub done: bool,
}
