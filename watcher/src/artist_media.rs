use serde::Deserialize;

use crate::artist::Artist;

#[derive(Deserialize)]
pub struct ArtistMedia {
    pub artist: Artist,
    pub media: Vec<u64>
}