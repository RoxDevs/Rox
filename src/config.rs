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

#[derive(Debug, PartialEq, Clone)]
pub struct Config {
    pub path: PathBuf,
    pub target: String,
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
        let config: Config = RawConfig::from_str(include_str!(r#"..\configs\example.toml"#))
            .unwrap()
            .into();
        assert_eq!(
            config,
            Config {
                path: PathBuf::from_str(r#"D:\rust proj\rox\packages"#).unwrap(),
                target: "x86_64-pc-windows-gnu".to_string()
            }
        )
    }
}
