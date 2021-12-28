use rocket::serde::{Serialize, Deserialize};

/// Database object for an Artist.
#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
#[serde(crate = "rocket::serde")]
#[table_name="artists"]
pub struct Artist {
    /// The artist's Twitter ID.
    #[serde(skip_deserializing)]
    pub twitter_id: Option<i64>,

    /// The artist's current Twitter username.
    pub twitter_username: String,

    /// The last uploaded media tweet that was seen.
    pub last_processed_tweet_id: Option<i64>
}

table! {
    pub artists (twitter_id) {
        twitter_id -> Nullable<BigInt>,
        twitter_username -> Text,
        last_processed_tweet_id -> Nullable<BigInt>,
    }
}