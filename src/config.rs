use std::path::Path;
use std::io::{Read, BufReader};
use std::fs::File;

use serde_derive::Deserialize;

// This is what we're going to decode into.
#[derive(Deserialize)]
pub struct Config {
    pub bridge_ip_address: String,
    pub username: String,
}

// Parse config file.
pub fn parse_config(config_path: &Path) -> Result<Config, failure::Error> {

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
        assert_eq!(config.bridge_ip_address, "192.168.1.10");
        assert_eq!(config.username, "3AyHHXYqfsEaWTD102MLlDNeBiJkbuk6XY8YOqK1");
    }
    match parse_config(Path::new("./data/config.toml")) {
        Ok(config) => test_config(config),
        Err(error) => panic!("Error has occurred in test_parse_config(): {:?}", error),
    };
}