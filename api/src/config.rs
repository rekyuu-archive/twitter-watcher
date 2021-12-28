use std::env;

pub struct Config {
    pub api_token: String,
    pub twitter_consumer_key: String,
    pub twitter_consumer_secret: String,
    pub twitter_access_key: String,
    pub twitter_access_secret: String,
}

impl Config {
    pub fn new() -> Self {
        Config {
            api_token: match env::var("API_TOKEN") {
                Ok(v) => v,
                Err(_e) => panic!("API_TOKEN is not set.")
            },

            twitter_consumer_key: match env::var("TWITTER_CONSUMER_KEY") {
                Ok(v) => v,
                Err(_e) => panic!("TWITTER_CONSUMER_KEY is not set.")
            },

            twitter_consumer_secret: match env::var("TWITTER_CONSUMER_SECRET") {
                Ok(v) => v,
                Err(_e) => panic!("TWITTER_CONSUMER_SECRET is not set.")
            },

            twitter_access_key: match env::var("TWITTER_ACCESS_TOKEN") {
                Ok(v) => v,
                Err(_e) => panic!("TWITTER_ACCESS_TOKEN is not set.")
            },

            twitter_access_secret: match env::var("TWITTER_ACCESS_SECRET") {
                Ok(v) => v,
                Err(_e) => panic!("TWITTER_ACCESS_SECRET is not set.")
            }
        }
    }
}