use reqwest::blocking::Client;
use serde::Deserialize;
use log::info; // Added logging
#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
pub struct Repo {
    pub name: String,
    pub readme: Option<String>,
}

impl Repo {
    pub fn readme(&self) -> Option<&str> {
        self.readme.as_deref()
    }
}

#[derive(Debug)]
pub struct GithubUser {
    pub username: String,
    pub repos: Vec<Repo>,
}

impl GithubUser {
    /// Create a new GithubUser by fetching the repositories and their READMEs
    pub fn new(client: &Client, username: &str) -> Self {
        info!("Creating GithubUser for username: {}", username); // Logging added
        let url = format!("https://api.github.com/users/{username}/repos?sort=updated");
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::USER_AGENT,
            reqwest::header::HeaderValue::from_static("rust-github-client"),
        );
        info!("Fetching repositories for user: {}", username); // Logging added
        let res = client
            .get(&url)
            .headers(headers)
            .send()
            .expect("Failed to send request");

        let repos: Vec<Repo> = res
            .json::<Vec<Repo>>()
            .expect("Failed to parse response")
            .iter()
            .map(|repo| {
                let mut repo = repo.clone();
                info!("Fetching README for repository: {}", repo.name); // Logging added
                let readme_md_url = format!(
                    "https://raw.githubusercontent.com/{}/{}/master/README.md",
                    username, repo.name
                );
                let readme_res = client.get(&readme_md_url).send();

                match readme_res {
                    Ok(readme) => {
                        if readme.status().is_success() {
                            let mut readme_content = readme.text().expect("Failed to read README.md");
                            if readme_content.len() > 500 {
                                readme_content.truncate(500);
                                readme_content.push_str("... Shortened for AI to read");
                            }
                            repo.readme = Some(readme_content);
                            info!("Successfully fetched README.md for repository: {}", repo.name); // Logging added
                        } else {
                            // Try fetching README if README.md isn't available
                            let readme_url = format!(
                                // e.g. 
                                "https://raw.githubusercontent.com/{}/{}/master/README",
                                username, repo.name
                            );
                            let readme_res = client.get(&readme_url).send();
                            if let Ok(readme) = readme_res {
                                if readme.status().is_success() {
                                    let mut readme_content = readme.text().expect("Failed to read README");
                                    if readme_content.len() > 500 {
                                        readme_content.truncate(500);
                                        readme_content.push_str("... Shortened for AI to read");
                                    }
                                    repo.readme = Some(readme_content);
                                    info!("Successfully fetched README for repository: {}", repo.name); // Logging added
                                } else {
                                    repo.readme = None;
                                    info!("No README found for repository: {}", repo.name); // Logging added
                                }
                            } else {
                                repo.readme = None;
                                info!("Failed to fetch README for repository: {}", repo.name); // Logging added
                            }
                        }
                    }
                    Err(_) => {
                        repo.readme = None;
                        info!("Error occurred while fetching README for repository: {}", repo.name); // Logging added
                    }
                }
                repo
            })
            .collect();

        info!("Successfully created GithubUser for username: {}", username); // Logging added
        GithubUser { username: username.to_owned(), repos }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::blocking::Client;

    const TEST_USERNAME: &str = "ofluffydev";

    fn dummy_repo(name: &str, readme: Option<&str>) -> Repo {
        Repo {
            name: name.to_string(),
            readme: readme.map(|s| s.to_string()),
        }
    }

    fn dummy_user() -> GithubUser {
        GithubUser {
            username: TEST_USERNAME.to_string(),
            repos: vec![
                dummy_repo("Hello-World", Some("# Hello")),
                dummy_repo("Spoon-Knife", None),
            ],
        }
    }

    #[test]
    fn test_repo_name() {
        let repo = dummy_repo("example-repo", Some("README"));
        assert_eq!(repo.name, "example-repo");
    }

    #[test]
    fn test_repo_readme_some() {
        let repo = dummy_repo("example-repo", Some("This is a README"));
        assert_eq!(repo.readme(), Some("This is a README"));
    }

    #[test]
    fn test_repo_readme_none() {
        let repo = dummy_repo("example-repo", None);
        assert_eq!(repo.readme(), None);
    }

    #[test]
    fn test_github_user_username() {
        let user = dummy_user();
        assert_eq!(user.username, TEST_USERNAME);
    }

    #[test]
    fn test_github_user_repos_len() {
        let user = dummy_user();
        assert_eq!(user.repos.len(), 2);
    }

    #[test]
    fn test_github_user_live_fetch() {
        let client = Client::new();
        let user = GithubUser::new(&client, TEST_USERNAME);
        assert_eq!(user.username, TEST_USERNAME);
        assert!(!user.repos.is_empty());
    }
}
