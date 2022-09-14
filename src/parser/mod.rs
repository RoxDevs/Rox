use serde::Deserialize;
use toml::from_str;

// This is until the CLI is implemented
#[allow(dead_code)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Deserialize)]
pub struct Project {
    tarball: String,
    dependencies: Vec<String>,
    mac: bool,
    win: bool,
    linux: bool,
    major: String,
    minor: String,
    rev: String,
}

impl Project {
    pub fn create_from_file(name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = std::fs::read_to_string(name)?;
        Ok(Project::create_from_str(file.as_str())?.clone())
    }
    fn create_from_str(contents: &str) -> Result<Project, Box<dyn std::error::Error>> {
        Ok(from_str::<Project>(contents)?.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_tests() {
        let toml = include_str!("../../tomls/basic.toml");
        let project = Project::create_from_str(toml).unwrap();
        assert_eq!(
            project,
            Project {
                tarball: "www".to_string(),
                dependencies: vec![],
                mac: true,
                win: false,
                linux: true,
                major: "0".to_string(),
                minor: "1".to_string(),
                rev: "0".to_string()
            }
        )
    }
}
