mod csv_manager;
mod ratings;

use anyhow::Result;
use csv_manager::{read_ratings, write_ratings};
use ratings::{Rating, RatingType};

fn main() -> Result<()>{
    let ratings = read_ratings("./db/album.csv")?;

    for rating in ratings {
        println!("{}", rating)
    }

    let rating = Rating{
        name: String::from("IGOR"),
        icon: String::from("🩷💔🩶"),
        rating: 8,
        rating_type: RatingType::Album{
            artist: String::from("Tyler, The Creator"),
        },
    };

    write_ratings("./db/album.csv", &rating)?;

    Ok(())
}
