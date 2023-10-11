use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckAccessibilityRequest {
    pub source_code: String,
    pub stream: Option<bool>,
}