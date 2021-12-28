use std::thread::sleep;
use std::time;
use log::{info, warn};
use simple_logger::SimpleLogger;

mod artist;
mod artist_media;
mod config;

use crate::artist_media::ArtistMedia;
use crate::config::Config;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    SimpleLogger::new().with_utc_timestamps().init().unwrap();

    let wait_duration = time::Duration::from_secs(5 * 60);
    let client = reqwest::Client::new();

    loop {
        info!("Pulling artists...");

        let artists = get_artist_media(&client).await?;

        for artist in artists {
            info!("  Processing {}...", artist.artist.twitter_username);

            if artist.media.len() > 0 {
                for tweet_id in artist.media {
                    let twitter_url = format!("https://twitter.com/{}/status/{}", artist.artist.twitter_username, tweet_id);

                    send_telegram_message(&client, twitter_url.as_str()).await?;
                }
            }
        }

        info!("Finished processing.");

        sleep(wait_duration);
    }
}

async fn get_artist_media(client: &reqwest::Client) -> Result<Vec<ArtistMedia>, reqwest::Error> {
    let config = Config::new();

    let body = client.get(format!("{}/api", config.api_url))
        .header("X-ACCESS-TOKEN", config.api_token)
        .send()
        .await;

    let result = match body {
        Ok(r) => r.json::<Vec<ArtistMedia>>().await?,
        Err(e) => {
            warn!("Pulling artists failed: {:?}", e);
            Vec::new()
        },
    };

    return Ok(result);
}

async fn send_telegram_message(client: &reqwest::Client, message: &str) -> Result<(), reqwest::Error> {
    let time_between_messages = time::Duration::from_secs(3);

    let config = Config::new();

    let telegram_url = format!("https://api.telegram.org/bot{}:{}/sendMessage?chat_id={}&text={}",
                               config.telegram_bot_id, config.telegram_bot_token, config.telegram_chat_id, message);

    client.post(telegram_url)
        .send()
        .await?;

    sleep(time_between_messages);

    Ok(())
}