use reqwest::blocking::Client;
#[allow(dead_code)]
#[derive(Debug)]
pub struct Repo {
    name: String,
    readme: Option<String>,
}

impl Repo {
    /// Get the name of the repository
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the README contents if available
    pub fn readme(&self) -> Option<&str> {
        self.readme.as_deref()
    }
}

#[derive(Debug)]
pub struct GithubUser {
    username: String,
    repos: Vec<Repo>,
}

impl GithubUser {
    /// Create a new GithubUser by fetching the repositories and their READMEs
    pub fn new(client: &Client, username: &str) -> Self {
        todo!("Fetch the list of repositories and populate with README if available");

        // Placeholder to make it compile
        GithubUser {
            username: username.to_string(),
            repos: Vec::new(),
        }
    }

    /// Get the username
    pub fn username(&self) -> &str {
        &self.username
    }

    /// Get a reference to all repositories
    pub fn repos(&self) -> &[Repo] {
        &self.repos
    }

    /// Get a repository by name (case-sensitive)
    pub fn get_repo(&self, name: &str) -> Option<&Repo> {
        todo!("Search and return the repository by name")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::blocking::Client;

    const TEST_USERNAME: &str = "octocat";

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
        assert_eq!(repo.name(), "example-repo");
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
        assert_eq!(user.username(), TEST_USERNAME);
    }

    #[test]
    fn test_github_user_repos_len() {
        let user = dummy_user();
        assert_eq!(user.repos().len(), 2);
    }

    #[test]
    fn test_github_user_get_repo_found() {
        let user = dummy_user();

        let repo = user.get_repo("Hello-World");
        assert!(repo.is_some());
        assert_eq!(repo.unwrap().name(), "Hello-World");
    }

    #[test]
    fn test_github_user_get_repo_not_found() {
        let user = dummy_user();

        let repo = user.get_repo("Nonexistent");
        assert!(repo.is_none());
    }
    // Optional: Integration test shell (would require actual logic to be implemented)
    #[test]
    #[ignore] // remove this after implementing GithubUser::new
    fn test_github_user_live_fetch() {
        let client = Client::new();
        let user = GithubUser::new(&client, TEST_USERNAME);
        assert_eq!(user.username(), TEST_USERNAME);
        assert!(!user.repos().is_empty());
    }
}
