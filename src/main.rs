mod csv_parser;
mod ratings;

use anyhow::Result;
use csv_parser::read_ratings;

fn main() -> Result<()>{
    let ratings = read_ratings("./db/albums.csv")?;

    for rating in ratings {
        println!("{}", rating)
    }

    Ok(())
}
