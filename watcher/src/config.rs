use std::env;

pub struct Config {
    pub api_token: String,
    pub api_url: String,
    pub telegram_bot_id: String,
    pub telegram_bot_token: String,
    pub telegram_chat_id: String,
}

impl Config {
    pub fn new() -> Self {
        Config {
            api_token: match env::var("API_TOKEN") {
                Ok(v) => v,
                Err(_e) => panic!("API_TOKEN is not set.")
            },

            api_url: match env::var("API_URL") {
                Ok(v) => v,
                Err(_e) => "http://localhost:5080".to_string()
            },

            telegram_bot_id: match env::var("TELEGRAM_BOT_ID") {
                Ok(v) => v,
                Err(_e) => panic!("TELEGRAM_BOT_ID is not set.")
            },

            telegram_bot_token: match env::var("TELEGRAM_BOT_TOKEN") {
                Ok(v) => v,
                Err(_e) => panic!("TELEGRAM_BOT_TOKEN is not set.")
            },

            telegram_chat_id: match env::var("TELEGRAM_CHAT_ID") {
                Ok(v) => v,
                Err(_e) => panic!("TELEGRAM_CHAT_ID is not set.")
            },
        }
    }
}