use async_trait::async_trait;
use regex::Regex;
use rocket::{Rocket, Build, Request};
use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::response::{Debug, status::Created};
use rocket::serde::json::Json;

use rocket_sync_db_pools::diesel;
use crate::config::Config;

use self::diesel::prelude::*;

use crate::models::artist::*;
use crate::models::artist_media::*;
use crate::twitter_api;

/// The sqlite database connection.
#[database("db")]
struct Db(diesel::SqliteConnection);

/// Return object for endpoints.
type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

/// Struct used for authorization headers.
struct AccessToken(String);

/// Failure states for invalid AccessTokens.
#[derive(Debug)]
enum AccessTokenError {
    Missing,
    Invalid
}

#[async_trait]
impl<'r> FromRequest<'r> for AccessToken {
    type Error = AccessTokenError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let token = request.headers().get_one("X-ACCESS-TOKEN");

        match token {
            Some(token) => {
                let config = Config::new();
                let expected_token = config.api_token;

                // Ensure the X-ACCESS-TOKEN header matches the supplied API_TOKEN.
                if token == expected_token {
                    Outcome::Success(AccessToken(token.to_string()))
                }
                else {
                    Outcome::Failure((Status::Unauthorized, AccessTokenError::Invalid))
                }
            },
            None => Outcome::Failure((Status::Unauthorized, AccessTokenError::Missing))
        }
    }
}

/// Gets a list of all artists and their uploaded media since their last updates.
#[get("/?<media>")]
async fn get_artists(_token: AccessToken, db: Db, media: Option<bool>) -> Result<Json<Vec<ArtistMedia>>> {
    let artist_profiles: Vec<Artist> = db.run(move |conn| {
        artists::table.load(conn)
    }).await?;

    let mut artist_medias: Vec<ArtistMedia> = artist_profiles
        .iter()
        .map(|a| ArtistMedia::new(a.clone()))
        .collect();

    if !media.is_none() && media.unwrap() == false {
        return Ok(Json(artist_medias));
    }

    for artist in &mut artist_medias {
        let artist_profile = twitter_api::get_account_by_id(artist.artist.twitter_id.unwrap() as u64).await;
        let current_username = artist_profile.screen_name;

        artist.artist.twitter_username = current_username.clone();

        artist.grab_media().await;

        let artist_twitter_id = artist.artist.twitter_id;
        let last_tweet_id = artist.artist.last_processed_tweet_id;

        if artist.media.len() > 0 {
            db.run(move |conn| {
                diesel::update(artists::table
                    .filter(artists::twitter_id.eq(&artist_twitter_id)))
                    .set((
                        artists::last_processed_tweet_id.eq(last_tweet_id),
                        artists::twitter_username.eq(current_username)))
                    .execute(conn)
                    .expect("Failed to update artist");
            }).await;
        }
    }

    return Ok(Json(artist_medias));
}

/// Adds a new artist to the database.
#[post("/?<url>")]
async fn create_artist(_token: AccessToken, db: Db, url: &str) -> Result<Created<Json<Artist>>> {
    let re = Regex::new(r"https?://(?:www\.)?twitter\.com/(?P<username>[a-zA-Z0-9_]+)(?:/.*)?").unwrap();
    let captures = re.captures(url).unwrap();
    let username = captures.name("username").unwrap().as_str();

    let twitter_account = twitter_api::get_account_by_name(String::from(username)).await;
    let media_tweets = twitter_api::get_user_media_tweets(twitter_account.id, None).await;
    let last_media_tweet_id = media_tweets.last().unwrap();

    let artist = Artist {
        twitter_id: Option::from(twitter_account.id as i64),
        twitter_username: String::from(username),
        last_processed_tweet_id: Option::from(last_media_tweet_id.clone() as i64)
    };

    let artist_values = artist.clone();

    db.run(move |conn| {
        diesel::insert_into(artists::table)
            .values(&artist_values)
            .execute(conn)
    }).await?;

    return Ok(Created::new("/").body(Json(artist)));
}

/// Removes an artist from the database.
#[delete("/<id>")]
async fn delete_artist(_token: AccessToken, db: Db, id: i64) -> Result<Option<()>> {
    let artist = db.run(move |conn| {
        diesel::delete(artists::table)
            .filter(artists::twitter_id.eq(id))
            .execute(conn)
    }).await?;

    return Ok((artist == 1).then(|| ()));
}

/// Runs the database migrations on startup.
async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    embed_migrations!("migrations");

    let conn = Db::get_one(&rocket).await.expect("database connection");
    conn.run(|c| embedded_migrations::run(c)).await.expect("diesel migrations");

    return rocket;
}

/// Staging method used to bootstrap the database and the controller endpoints.
pub fn stage() -> AdHoc {
    return AdHoc::on_ignite("Diesel SQLite Stage", |rocket| async {
        rocket.attach(Db::fairing())
            .attach(AdHoc::on_ignite("Diesel Migrations", run_migrations))
            .mount("/api", routes![get_artists, create_artist, delete_artist])
    });
}