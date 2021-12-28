use serde::Deserialize;

#[derive(Deserialize)]
pub struct Artist {
    pub twitter_id: Option<i64>,
    pub twitter_username: String,
    pub last_processed_tweet_id: Option<i64>
}