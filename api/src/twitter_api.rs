use egg_mode::Token;
use egg_mode::user::TwitterUser;
use egg_mode::tweet::Tweet;

use crate::config::Config;

/// Gets an account object by username.
pub async fn get_account_by_name(username: String) -> TwitterUser {
    let token = get_token();

    let user = egg_mode::user::show(username, &token).await.unwrap();

    return user.response;
}

/// Gets an account object by user ID.
pub async fn get_account_by_id(user_id: u64) -> TwitterUser {
    let token = get_token();

    let user = egg_mode::user::show(user_id, &token).await.unwrap();

    return user.response;
}

/// Gets a list of uploaded media tweets from a user.
/// Can optionally supply a tweet ID to specify to return only new tweets since that ID.
pub async fn get_user_media_tweets(user_id: u64, since_id: Option<u64>) -> Vec<u64> {
    let mut media_tweets: Vec<u64> = Vec::new();
    let token = get_token();

    let timeline = egg_mode::tweet::user_timeline(user_id, false, false, &token).with_page_size(100);
    let (_timeline, feed) = timeline.older(since_id).await.unwrap();

    let mut tweets: Vec<Tweet> = feed.response;

    tweets.reverse();

    for tweet in tweets {
        if tweet.entities.media.is_some() {
            media_tweets.push(tweet.id);
        }
    }

    return media_tweets;
}

/// Creates a Twitter API token.
fn get_token() -> Token {
    let config = Config::new();

    let consumer_token = egg_mode::KeyPair::new(config.twitter_consumer_key, config.twitter_consumer_secret);
    let access_token = egg_mode::KeyPair::new(config.twitter_access_key, config.twitter_access_secret);

    return egg_mode::Token::Access {
        consumer: consumer_token,
        access: access_token
    };
}