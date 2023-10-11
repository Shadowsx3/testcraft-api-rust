use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GenerateIdeasRequest {
    #[serde(rename = "sourceCode")]
    pub source_code: String,
    pub stream: Option<bool>,
}