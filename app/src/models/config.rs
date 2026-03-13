use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    pub cookie: String,
    pub bot_prompt: String,
    pub manger_qq_id: u64,
    pub host: String,
    pub prot: u64,
}

impl AppConfig {
    pub fn new() -> Self {
        Self {
            cookie: "".to_string(),
            bot_prompt: "".to_string(),
            manger_qq_id: 0,
            host: "0.0.0.0".to_string(),
            prot: 5410,
        }
    }
}
