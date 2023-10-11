use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatChunkDelta {
    pub content: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatChunkChoice {
    pub delta: ChatChunkDelta,
    pub index: usize,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatCompletionChunk {
    pub id: String,
    pub object: String,
    pub created: usize,
    pub model: String,
    pub choices: Vec<ChatChunkChoice>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatCompletionChunkResponse {
    pub choices: Vec<ChatChunkChoice>,
}