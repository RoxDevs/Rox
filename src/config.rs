use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Config {
    pub path: String,
    pub target: String,
}

impl Config {
    pub fn from_str(config: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        let config = Config::from_str(include_str!("../configs/example.toml")).unwrap();
        assert_eq!(
            config,
            Config {
                path: "/home/muppi/.rox".to_string(),
                target: "x86_64-unknown-linux-gnu".to_string()
            }
        )
    }
}
