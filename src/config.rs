use serde::Deserialize;
use std::{path::PathBuf, str::FromStr};

#[derive(Debug, Deserialize, PartialEq)]
pub struct RawConfig {
    path: String,
    target: String,
}

impl RawConfig {
    pub fn from_str(config: &str) -> Result<Self, toml::de::Error> {
        toml::from_str::<RawConfig>(config)
    }
}

#[derive(Debug, PartialEq)]
pub struct Config {
    path: PathBuf,
    target: String,
}

impl Into<Config> for RawConfig {
    fn into(self) -> Config {
        Config {
            path: PathBuf::from_str(self.path.as_str()).unwrap(),
            target: self.target,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        let config: Config = RawConfig::from_str(include_str!("../configs/example.toml")).unwrap().into();
        assert_eq!(
            config,
            Config {
                path: PathBuf::from_str("/home/muppi/.rox/").unwrap(),
                target: "x86_64-unknown-linux-gnu".to_string()
            }
        )
    }
}
