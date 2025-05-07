mod csv_handler;
mod ratings;
mod cli;

use anyhow::{Result, anyhow};
use clap::Parser;
use csv_handler::{read_ratings, write_ratings};
use ratings::{Rating, RatingType};
use cli::{Cli, Commands};

fn main() -> Result<()>{
    // let ratings = read_ratings("./db/album.csv")?;

    // for rating in ratings {
    //     println!("{}", rating)
    // }

    // let rating = Rating{
    //     name: String::from("IGOR"),
    //     icon: String::from("🩷💔🩶"),
    //     rating: 8,
    //     rating_type: RatingType::Album{
    //         artist: String::from("Tyler, The Creator"),
    //     },
    // };

    // write_ratings("./db/album.csv", &rating)?;

    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { category, name, icon, rating, properties} => {
            let rating_type = handle_category(category, properties)?;
            let rating = Rating {
                name: name.clone(),
                icon: icon.clone(),
                rating: *rating,
                rating_type: rating_type
            };
            println!("{}", rating);
            write_ratings(format!("./db/{}.csv", category), &rating)?;
        }
    }

    Ok(())
}

fn handle_category(category: &String, properties: &Vec<String>)  -> Result<RatingType> {
    match category.as_str() {
        "album" => Ok(RatingType::Album {
            artist: properties[0].clone()
        }),
        _ => Err(anyhow!("Invalid category: {}", category))
    }
}
