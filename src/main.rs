// #![deny(warnings)]

use std::{env::args, error::Error, time::Duration};

use github::Repo;
use reqwest::blocking;
mod github;
mod rating;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init(); // Ignore this

    let username = match args().nth(1) {
        Some(username) => username,
        None => {
            eprintln!("Usage: {} <username>", args().next().unwrap());
            std::process::exit(1);
        }
    };

    let url = format!("https://api.github.com/users/{username}/repos");

    // Let the user know we are about to fetch the URL
    println!("Fetching {username:?}...");

    /*
       Blocking simply means it holds up the entire program until it's finished
       Typically, you'll want to do non-blocking, but in our case, we can't
       continue without the results anyway, so this is fine.
    */

    // Create a "blocking" client that timesout after 10 seconds of waiting.
    let client = blocking::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;

    /*
        Send a request, "get()" specifies what URL to use, and you can specify other
        options like headers, HTTP methods, etc. before using .send() to finally
        use the request.
    */
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::USER_AGENT,
        reqwest::header::HeaderValue::from_static("rust-github-client"),
    );
    let mut res = client.get(&url).headers(headers).send().expect("Failed to send request");

    let mut repos = res.json::<Vec<Repo>>()?;
    for repo in &mut repos {
        println!("Name: {}", repo.name());
    }

    // Logs the HTTP version and status code returned from the request.
    // println!("Response: {:?} {}", res.version(), res.status());

    // Logs any headers returned.
    // println!("Headers: {:#?}\n", res.headers());

    // copy the response body directly to stdout (Like doing println but for the entire response)
    // res.copy_to(&mut std::io::stdout())?;

    Ok(())
}
