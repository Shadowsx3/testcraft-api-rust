use serde::Deserialize;
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutomateTestsRequest {
    pub source_code: String,
    pub base_url: String,
    pub framework: String,
    pub language: String,
    pub stream: Option<bool>,
    pub pom: Option<bool>,
}


#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutomateTestsIdeasRequest {
    pub source_code: String,
    pub base_url: String,
    pub framework: String,
    pub language: String,
    pub ideas: Vec<String>,
    pub stream: Option<bool>,
    pub pom: Option<bool>,
}
