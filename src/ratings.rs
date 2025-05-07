use serde::{Serialize, Deserialize};
use std::fmt;

#[derive(Debug, Deserialize, Serialize)]
pub struct Rating {
    pub name: String,
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
        match &self.rating_type {
            RatingType::Album { artist } => {
                write!(f, "{} by {}", self.name, artist)
            }
        }
    }
}

pub fn display_ratings(ratings: &Vec<Rating>) {
    let max_dislay_rating_len = ratings.iter().map(|r| format!("{}", r).len()).max().unwrap_or(0);

    for rating in ratings {
        let stars = format!(
            "{}{}",
            "★ ".repeat(rating.rating as usize),
            "☆ ".repeat(10 - rating.rating as usize)
        );
        let display_rating = format!("{}", rating);
        let padding = max_dislay_rating_len - display_rating.len();
        let format_rating = format!("{} {} {}", rating, " ".repeat(padding), stars);
        println!("{}\n{}", format_rating, "-".repeat(format_rating.len()))
    }
}
