use anyhow::{Context, Result};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
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

pub fn write_ratings<P: AsRef<Path>>(path: P, rating: &Rating) -> Result<()> {
    let ratings_file = OpenOptions::new().append(true).open(&path).with_context(|| format!("Failed to open ratings file: {}", path.as_ref().display()))?;
    let mut csv_writer = csv::WriterBuilder::new().has_headers(false).from_writer(ratings_file);

    csv_writer.serialize(&rating.to_csv_record()).with_context(|| format!("Failed to serialize a record to the CSV"))?;
    csv_writer.flush().with_context(|| format!("Failed to flush CSV file"))?;
    Ok(())
}

pub fn edit_rating<P: AsRef<Path>>(path: P, rating: &Rating) -> Result<()> {
    let ratings_file = File::open(&path).with_context(|| format!("Failed to open ratings file: {}", path.as_ref().display()))?;
    let reader = BufReader::new(ratings_file);

    let lines: Vec<String>= reader
    .lines()
    .filter_map(Result::ok)
    .filter(|line| !line.starts_with(&rating.name))
    .collect();

    let mut ratings_file = File::create(&path).with_context(|| format!("Failed to create ratings file: {}", path.as_ref().display()))?;
    for line in lines {
        // TODO: add context to all unwraps
        writeln!(ratings_file, "{}", line)?;
    }

    write_ratings(path, rating)
}
