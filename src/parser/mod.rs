use serde::Deserialize;
use toml::from_str;

// This is until the CLI is implemented
#[allow(dead_code)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Deserialize)]
pub struct RawProject {
    dependencies: Vec<String>,
    mac: Option<String>,
    win: Option<String>,
    linux: Option<String>,
    major: String,
    minor: String,
    rev: String,
}

// Until the CLI is built
#[allow(dead_code)]
impl RawProject {
    pub fn create_from_file(name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = std::fs::read_to_string(name)?;
        Ok(RawProject::create_from_str(file.as_str())?.clone())
    }
    fn create_from_str(contents: &str) -> Result<RawProject, Box<dyn std::error::Error>> {
        Ok(from_str::<RawProject>(contents)?.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_tests() {
        let toml = include_str!("../../tomls/basic.toml");
        let project = RawProject::create_from_str(toml).unwrap();
        assert_eq!(
            project,
            RawProject {
                dependencies: vec![],
                mac: Some("mac".to_string()),
                win: None,
                linux: Some("linux".to_string()),
                major: "0".to_string(),
                minor: "1".to_string(),
                rev: "0".to_string()
            }
        )
    }
}
