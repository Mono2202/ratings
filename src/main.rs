mod csv_parser;

use anyhow::Result;
use csv_parser::read_ratings;

fn main() -> Result<()>{
    let ratings = read_ratings("./db/albums.csv")?;
    println!("{:?}", ratings);

    Ok(())
}
