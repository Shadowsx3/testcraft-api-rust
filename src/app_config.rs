pub struct AppState {
    pub env: Config,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub open_ai_key: String,
}

impl Config {
    pub fn init() -> Config {
        let open_ai_key = std::env::var("OPEN_AI_KEY").expect("OPEN_AI_KEY must be set");
        Config {
            open_ai_key,
        }
    }
}