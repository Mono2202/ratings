use anyhow::{Context, Result};
use std::fs::File;
use std::path::Path;
use csv;

use crate::ratings::Rating;

pub fn read_ratings<P: AsRef<Path>>(path: P) -> Result<Vec<Rating>> {
    let ratings_file = File::open(&path).with_context(|| format!("Failed to open ratings file: {}", path.as_ref().display()))?;
    let mut csv_reader = csv::Reader::from_reader(ratings_file);
    let mut ratings = Vec::new();

    for record in csv_reader.deserialize() {
        let rating = record.with_context(|| format!("Failed to parse a record from the CSV"))?;
        ratings.push(rating);
    }

    Ok(ratings)
}
