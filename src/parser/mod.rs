use serde::Deserialize;
use toml::from_str;

// This is until the CLI is implemented
#[allow(dead_code)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Deserialize)]
pub struct RawVer {
    dependencies: Vec<String>,
    mac: Option<String>,
    win: Option<String>,
    linux: Option<String>,
    version: String,
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
            mac: self.mac,
            win: self.win,
            linux: self.linux,
            major: version[0],
            minor: version[1],
            rev: version[2],
        }
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Ver {
    dependencies: Vec<Vec<String>>,
    mac: Option<String>,
    win: Option<String>,
    linux: Option<String>,
    major: usize,
    minor: usize,
    rev: usize,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Deserialize)]
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

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
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
                mac: Some("mac".to_string()),
                win: None,
                linux: Some("linux".to_string()),
                version: "0.1.0".to_string(),
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
                mac: Some("mac".to_string()),
                win: None,
                linux: Some("linux".to_string()),
                major: 0,
                minor: 1,
                rev: 0,
            }
        )
    }

    #[test]
    fn source_tests() {
        let toml = include_str!("../../tomls/source.toml");
        let project: Ver = RawVer::create_from_str(toml).unwrap().into();
        assert_eq!(
            project,
            Ver {
                dependencies: vec![],
                mac: None,
                linux: None,
                major: 0,
                minor: 1,
                rev: 0,
                win: None
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
                        mac: Some("mac".to_string()),
                        win: None,
                        linux: Some("linux".to_string()),
                        major: 0,
                        minor: 1,
                        rev: 0,
                    },
                    Ver {
                        dependencies: vec![vec!["yyy".to_string(), "yyy".to_string()]],
                        mac: Some("mac".to_string()),
                        win: None,
                        linux: Some("linux".to_string()),
                        major: 0,
                        minor: 1,
                        rev: 1,
                    }
                ],
                git: None,
                name: "Rox".to_string(),
                authors: vec!["muppi090909".to_string()]
            }
        );
    }
}
