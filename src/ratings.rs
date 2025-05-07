use serde::{Serialize, Deserialize};
use clap::ValueEnum;
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
#[serde(tag="type")]
pub enum RatingType {
    #[serde(rename="album")]
    Album {
        artist: String,
    },
}

// Unfortunately, `serde(flatten)` doesn't work with CSV serialization.
// That is why this workaround is needed :(
impl Rating {
    pub fn to_csv_record(&self) -> Vec<String> {
        let mut csv_record = vec![
            self.name.clone(),
            self.icon.clone(),
            self.rating.to_string(),
        ];

        match &self.rating_type {
            RatingType::Album {artist} => {
                csv_record.push(artist.clone());
                csv_record.push("album".into());
            }
        }

        csv_record
    }
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
