use serde::Deserialize;
use toml::from_str;

// This is until the CLI is implemented
#[allow(dead_code)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Deserialize)]
pub struct Project<'scope> {
    tarball: &'scope str,
    dependencies: Vec<&'scope str>,
    mac: bool,
    win: bool,
    linux: bool,
    major: &'scope str,
    minor: &'scope str,
    rev: &'scope str,
}

impl<'scope> Project<'scope> {
    pub fn create_from_file(name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = std::fs::read_to_string(name)?;
        Ok(Project::create_from_str(file.as_str())?.clone())
    }
    fn create_from_str(contents: &'scope str) -> Result<Project<'scope>, Box<dyn std::error::Error>> {
        Ok(from_str::<Project<'_>>(contents)?.clone())
    }
}
