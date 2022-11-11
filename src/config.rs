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
        #[cfg(target_os = "linux")]
        let config: Config = RawConfig::from_str(include_str!("../configs/example.toml"))

            .unwrap()
            .into();
        #[cfg(target_os = "windows")]
        let config: Config = RawConfig::from_str(include_str!(r#"..\configs\win_example.toml"#))
            .unwrap()
            .into();
        #[cfg(target_os = "windows")]
        assert_eq!(
            config,
            Config {
                path: PathBuf::from_str(r#"D:\rust proj\rox\packages"#).unwrap(),
                target: "x86_64-pc-windows-gnu".to_string()
            }
        );
        #[cfg(target_os = "linux")]
        assert_eq!(
            config,
            Config {
                path: PathBuf::from_str(r#"/home/muppi/.rox/"#).unwrap(),
                target: "x86_64-unknown-linux-gnu".to_string()
            }
        )
    }
}
