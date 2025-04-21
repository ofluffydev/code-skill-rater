#![deny(warnings)]

use std::{env::args, error::Error};

use github::GithubUser;
use log::{error, info};
use rating::gen_rating;
use reqwest::blocking;
mod github;
mod rating;

fn main() -> Result<(), Box<dyn Error>> {
    let env_path = "/home/fluffy/Code/code-skill-rater/.env";
    dotenv::from_path(env_path).expect("Failed to load .env file");
    env_logger::init();

    let username = match args().nth(1) {
        Some(username) => username,
        None => {
            error!("Usage: {} <username>", args().next().unwrap());
            std::process::exit(1);
        }
    };

    info!("Fetching data for user: {}", username);

    let client = blocking::Client::builder()
        .timeout(None) // Set timeout to None for no timeout, AI might take a while
        .build()?;

    let user = GithubUser::new(&client, &username);
    let rating = gen_rating(&client, user);
    println!("Rating: {}", rating.description());

    Ok(())
}
