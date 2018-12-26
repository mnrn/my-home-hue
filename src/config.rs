use std::io::{Read, BufReader};
use std::fs::File;
use std::marker::PhantomData;
use std::default::Default;
use serde_derive::Deserialize;

// This is what we're going to decode into.
#[derive(Deserialize)]
pub struct Config {
    bridge_ip_address: String,
    username: String,
}

impl Config {
    pub fn bridge_ip_address(&self) -> &String {
        &self.bridge_ip_address
    }
    pub fn username(&self) -> &String {
        &self.username
    }
}

pub struct Empty;
pub struct Fully;
pub struct ConfigBuilder<ConfigPath> {
    config_path: String,
    state: PhantomData<ConfigPath>
}
impl ConfigBuilder<Empty> {
    pub fn new() -> ConfigBuilder<Empty> {
        ConfigBuilder { 
            config_path: Default::default(),
            state: PhantomData,
        }
    }
    pub fn config_path<S: Into<String>>(self, config_path: S) -> ConfigBuilder<Fully> {
        ConfigBuilder {
            config_path: config_path.into(),
            state: PhantomData,
        }
    }
}
impl ConfigBuilder<Fully> {
    // Parse config file.
    pub fn build(self) -> Result<Config, failure::Error> {
        let file = File::open(&self.config_path)?;
        let mut reader = BufReader::new(file);

        // read into a String.
        let mut buffer = String::new();
        reader.read_to_string(&mut buffer)?;

        let config = toml::from_str(&buffer)?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use crate::config::Config;
    use crate::config::ConfigBuilder;

    #[test]
    fn test_parse_config() {
        fn test_config(config: Config) {
            assert_eq!(config.bridge_ip_address(), "192.168.1.10");
            assert_eq!(config.username(), "3AyHHXYqfsEaWTD102MLlDNeBiJkbuk6XY8YOqK1");
        }

        match ConfigBuilder::new().config_path("./data/config.toml").build() {
            Ok(config) => test_config(config),
            Err(err) => panic!("Error has occurred in test_parse_config(): {:?}", err),
        };
    }
}