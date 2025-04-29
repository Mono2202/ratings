use serde::{Serialize, Deserialize};
use std::fmt;

#[derive(Debug, Deserialize, Serialize)]
pub struct Rating {
    pub name: String,
    pub icon: String,
    pub rating: i8,

    #[serde(flatten)]
    pub rating_type: RatingType,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum RatingType {
    Album {
        artist: String,
    },
}

impl fmt::Display for Rating {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rating_stars = "⭐".repeat(self.rating as usize);
        match &self.rating_type {
            RatingType::Album { artist } => {
                write!(f, "{} {} by {} [{}]", self.icon, self.name, artist, rating_stars)
            }
        }
    }
}