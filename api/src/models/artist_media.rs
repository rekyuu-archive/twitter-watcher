use rocket::serde::Serialize;

use crate::models::artist::Artist;
use crate::twitter_api;

/// Return object for the API for an artist and it's media.
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ArtistMedia {
    pub artist: Artist,
    pub media: Vec<u64>
}

impl ArtistMedia {
    /// Creates a new ArtistMedia object from an Artist database object.
    /// Instantiates media as empty. To populate media, `grab_media()` must be called.
    pub fn new(artist: Artist) -> Self {
        return ArtistMedia {
            artist,
            media: Vec::new()
        };
    }

    /// Gets all new media from an artist since the last update.
    pub async fn grab_media(&mut self) {
        let user_id = self.artist.twitter_id.unwrap() as u64;
        let last_tweet_id = Option::from(self.artist.last_processed_tweet_id.unwrap() as u64);
        let media_tweets = twitter_api::get_user_media_tweets(user_id, last_tweet_id).await;

        self.media = media_tweets.clone();

        match media_tweets.last() {
            Some(id) => self.artist.last_processed_tweet_id = Option::from(*id as i64),
            None => {}
        }
    }
}