CREATE TABLE artists (
    twitter_id PRIMARY KEY,
    twitter_username VARCHAR NOT NULL,
    last_processed_tweet_id INTEGER NULL
)