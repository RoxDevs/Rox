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
    version: String,
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

impl Into<Project> for RawProject {
    fn into(self) -> Project {
        let dependencies: Vec<Vec<String>> = self
            .dependencies
            .iter()
            .map(|item| item.split('/').map(|string| string.to_string()).collect())
            .collect();
        let version: Vec<usize> = self
            .version
            .split('.')
            .map(|int| int.parse::<usize>().expect("invalid SemVer"))
            .collect();
        assert_eq!(3, version.len(), "invalid SemVer");
        Project {
            dependencies,
            mac: self.mac,
            win: self.win,
            linux: self.linux,
            major: version[0],
            minor: version[1],
            rev: version[2],
        }
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Deserialize)]
pub struct Project {
    dependencies: Vec<Vec<String>>,
    mac: Option<String>,
    win: Option<String>,
    linux: Option<String>,
    major: usize,
    minor: usize,
    rev: usize,
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
                dependencies: vec!["xxx/yyy".to_string()],
                mac: Some("mac".to_string()),
                win: None,
                linux: Some("linux".to_string()),
                version: "0.1.0".to_string()
            }
        )
    }

    #[test]
    fn project_tests() {
        let toml = include_str!("../../tomls/basic.toml");
        let raw = RawProject::create_from_str(toml).unwrap();
        let project: Project = raw.into();
        assert_eq!(
            project,
            Project {
                dependencies: vec![vec!["xxx".to_string(), "yyy".to_string()]],
                mac: Some("mac".to_string()),
                win: None,
                linux: Some("linux".to_string()),
                major: 0,
                minor: 1,
                rev: 0
            }
        )
    }
}
