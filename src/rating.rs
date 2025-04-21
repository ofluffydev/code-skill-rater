use crate::github::GithubUser;

pub struct Rating {
    pub level: Level,
    pub description: String,
}

pub enum Level {
    NotAProgrammer,
    Beginner,
    Intermediate,
    Expert,
    Nerd,
}

impl Rating {
    pub fn new(level: Level, description: String) -> Self {
        Rating { level, description }
    }

    pub fn level(&self) -> &Level {
        &self.level
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}

pub fn gen_rating(user: GithubUser) -> Rating {
    todo!("Generate a rating based on the user's repositories and their READMEs");
}