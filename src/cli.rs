use clap::{Parser, Subcommand};
use crate::csv_handler::{edit_rating, read_ratings, write_ratings, delete_rating};
use crate::ratings::{display_ratings, Rating, RatingType};
use anyhow::{Result, anyhow};
use chrono::prelude::*;

#[derive(Parser)]
#[command(name="ratings")]
#[command(about="A CLI tool to keep ratings for albums, movies, etc.", long_about=None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Add {
        category: String,
        name: String,
        score: i8,
        properties: Vec<String>,
    },
    Show {
        category: String,

        #[arg(short, long)]
        sort: bool
    },
    Edit {
        category: String,
        name: String,
        score: i8
    },
    Delete {
        category: String,
        name: String,
    }
}

pub fn handle_command(command: &Commands) -> Result<()> {
    match command {
        Commands::Add {category, name, score, properties} => add_command(category, name, *score, properties),
        Commands::Show {category, sort} => show_command(category, *sort),
        Commands::Edit {category, name, score} => edit_command(category, name, *score),
        Commands::Delete {category, name} => delete_command(category, name),
    }
}

fn add_command(category: &String, name: &String, score: i8, properties: &Vec<String>) -> Result<()> {
    let rating_type = handle_category(category, properties)?;
    let rating = Rating {
        name: name.clone(),
        score: score,
        date: Local::now().format("%d/%m/%Y").to_string(),
        rating_type: rating_type
    };
    write_ratings(format!("./db/{}.csv", category), &rating)
}

fn show_command(category: &String, sort: bool) -> Result<()> {
    let mut ratings = read_ratings(format!("./db/{}.csv", category))?;
    if sort {
        ratings.sort_by_key(|r| std::cmp::Reverse(r.score))
    }

    display_ratings(&ratings);
    Ok(())
}

fn edit_command(category: &String, name: &String, score: i8) -> Result<()> {
            let mut ratings = read_ratings(format!("./db/{}.csv", category))?;
            for rating in ratings.iter_mut() {
                if &rating.name == name {
                    rating.score = score;
                    // TODO: not sure if I want to update the date
                    rating.date = Local::now().format("%d/%m/%Y").to_string();
                    edit_rating(format!("./db/{}.csv", category), &rating)?;
                    break
                }
            }

            Ok(())
}

fn delete_command(category: &String, name: &String) -> Result<()> {
            let mut ratings = read_ratings(format!("./db/{}.csv", category))?;
            for rating in ratings.iter_mut() {
                if &rating.name == name {
                    delete_rating(format!("./db/{}.csv", category), &rating)?;
                    break
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
