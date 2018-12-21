use std::path::Path;
use std::io::{Read, BufReader};
use std::fs::File;

use failure::Fail;
use serde_derive::Deserialize;

// This is what we're going to decode into.
#[derive(Debug, Deserialize)]
struct Config {
    bridge_ip_address: String,
    username: String,
}

#[derive(Debug, Fail)]
enum LoadConfigError {
    #[fail(display = "I/O Error: {:?}", error)]
    IOError {
        error: std::io::Error,
    },

    #[fail(display = "Decode Error: {:?}", error)]
    DecodeError {
        error: toml::de::Error,
    },
}

impl From<std::io::Error> for LoadConfigError {
    fn from(error: std::io::Error) -> Self {
        LoadConfigError::IOError { error }
    }
}

impl From<toml::de::Error> for LoadConfigError {
    fn from(error: toml::de::Error) -> Self {
        LoadConfigError::DecodeError { error }
    }
}

// Parse config file.
#[allow(dead_code)]
fn parse_config(config_path: &Path) -> Result<Config, LoadConfigError> {

    let file = File::open(config_path)?;
    let mut reader = BufReader::new(file);

    // read into a String.
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;

    let config = toml::from_str(&buffer)?;
    Ok(config)
}

#[test]
fn test_parse_config() {
    fn test_config(config: Config) {
        assert_eq!(config.bridge_ip_address, "192.168.1.72");
        assert_eq!(config.username, "testuser");
    }
    match parse_config(Path::new("./data/test_config.toml")) {
        Ok(config) => test_config(config),
        Err(error) => panic!("Error has occurred in test_parse_config(): {:?}", error),
    };
}