use log::info;
use reqwest::blocking::Client;
use serde::Deserialize;
use serde_json::json;

use crate::github::GithubUser;

pub struct Rating {
    pub description: String,
}

#[derive(Deserialize)]
pub struct ApiResponse {
    pub result: Result,
}

#[derive(Deserialize)]
pub struct Result {
    pub response: String,
}

impl Rating {
    pub fn new(description: String) -> Self {
        Rating { description }
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}

pub fn gen_rating(client: &Client, user: GithubUser) -> Rating {
    let account_id = std::env::var("ACCOUNT_ID").expect("ACCOUNT_ID not set");
    let api_key = std::env::var("API_KEY").expect("API_KEY not set");
    let url = format!(
        "https://api.cloudflare.com/client/v4/accounts/{account_id}/ai/run/@cf/meta/llama-3.1-8b-instruct"
    );

    const FULL_INSTRUCTIONS: &str = include_str!("../instructions.txt");

    let mut prompt = format!(
        "You are a professional programmer grader, currently reviewing the user \"{}\". {FULL_INSTRUCTIONS} \n\n",
        user.username,
    );

    for repo in user.repos {
        let readme = match repo.readme() {
            Some(readme) => readme,
            None => "No README available",
        };
        prompt.push_str(&format!(
            "Repository: {} \nREADME: {} \n\n",
            repo.name, readme
        ));
    }

    info!("Getting rating for user: {}", user.username);
    let res = client
        .post(&url) // Changed from `.get` to `.post`
        .header("Authorization", format!("Bearer {api_key}"))
        .header("Content-Type", "application/json")
        .json(&json!(
            {
                "messages": [
                    {
                        "role": "system",
                        "content": "You are a professional programmer grader. By default, you output to a terminal so do not use any markdown formatting."
                    },
                    {
                        "role": "user",
                        "content": prompt
                    }
                ],
                "max_tokens": 2000,
            }
        ))
        .send()
        .expect("Failed to send request");

    let api_response: ApiResponse = res.json().expect("Failed to parse response");

    Rating::new(api_response.result.response)
}
