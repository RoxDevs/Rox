use std::collections::HashMap;

use serde::Deserialize;
use toml::from_str;

// This is until the CLI is implemented
#[allow(dead_code)]
#[derive(Clone, PartialEq, Eq, Debug, Deserialize)]
pub struct RawVer {
    dependencies: Vec<String>,
    version: String,
    tarballs: HashMap<String, String>,
}

// Until the CLI is built
#[allow(dead_code)]
impl RawVer {
    pub fn create_from_file(name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = std::fs::read_to_string(name)?;
        Ok(RawVer::create_from_str(file.as_str())?.clone())
    }
    fn create_from_str(contents: &str) -> Result<RawVer, Box<dyn std::error::Error>> {
        Ok(from_str::<RawVer>(contents)?.clone())
    }
}

impl Into<Ver> for RawVer {
    fn into(self) -> Ver {
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
        Ver {
            dependencies,
            major: version[0],
            minor: version[1],
            rev: version[2],
            tarballs: self.tarballs,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Ver {
    dependencies: Vec<Vec<String>>,
    tarballs: HashMap<String, String>,
    major: usize,
    minor: usize,
    rev: usize,
}

#[derive(Clone, PartialEq, Eq, Debug, Deserialize)]
pub struct RawProject {
    git: Option<String>,
    name: String,
    #[serde(rename(serialize = "versions", deserialize = "version"))]
    versions: Vec<RawVer>,
    authors: Vec<String>,
}

// Until the CLI is built
#[allow(dead_code)]
impl RawProject {
    pub fn create_from_file(name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = std::fs::read_to_string(name)?;
        Ok(RawProject::create_from_str(file.as_str())?.clone())
    }
    fn create_from_str(contents: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(from_str::<RawProject>(contents)?.clone())
    }
}

impl Into<Project> for RawProject {
    fn into(self) -> Project {
        Project {
            git: self.git,
            name: self.name,
            versions: self
                .versions
                .iter()
                .cloned()
                .map(|ver| ver.into())
                .collect(),
            authors: self.authors,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Project {
    git: Option<String>,
    name: String,
    versions: Vec<Ver>,
    authors: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_tests() {
        let toml = include_str!("../../tomls/basic.toml");
        let project = RawVer::create_from_str(toml).unwrap();
        assert_eq!(
            project,
            RawVer {
                dependencies: vec!["xxx/yyy".to_string()],
                version: "0.1.0".to_string(),
                tarballs: HashMap::from([(
                    "x86_64-unknown-linux-gnu".to_string(),
                    "linux".to_string()
                )])
            }
        )
    }

    #[test]
    fn version_tests() {
        let toml = include_str!("../../tomls/basic.toml");
        let raw = RawVer::create_from_str(toml).unwrap();
        let project: Ver = raw.into();
        assert_eq!(
            project,
            Ver {
                dependencies: vec![vec!["xxx".to_string(), "yyy".to_string()]],
                major: 0,
                minor: 1,
                rev: 0,
                tarballs: HashMap::from([(
                    "x86_64-unknown-linux-gnu".to_string(),
                    "linux".to_string()
                )])
            }
        )
    }

    #[test]
    fn project_tests() {
        let toml = include_str!("../../tomls/simple-project.toml");
        let raw = RawProject::create_from_str(toml).unwrap();
        let project: Project = raw.into();
        assert_eq!(
            project,
            Project {
                versions: vec![
                    Ver {
                        dependencies: vec![vec!["xxx".to_string(), "yyy".to_string()]],
                        major: 0,
                        minor: 1,
                        rev: 0,
                        tarballs: HashMap::from([
                            ("x86_64-unknown-linux-gnu".to_string(), "linux".to_string()),
                            ("x86_64-pc-windows-gnu".to_string(), "windows".to_string())
                        ])
                    },
                    Ver {
                        dependencies: vec![vec!["xxx".to_string(), "yyy".to_string()]],
                        major: 0,
                        minor: 1,
                        rev: 1,
                        tarballs: HashMap::from([(
                            "x86_64-unknown-linux-gnu".to_string(),
                            "linux".to_string()
                        )])
                    }
                ],
                git: None,
                authors: vec!["muppi090909".to_string()],
                name: "Rox".to_string()
            }
        );
    }
}
