extern crate failure;
extern crate serde_derive;
extern crate serde_json;
extern crate toml;
extern crate reqwest;

use std::path::Path;

fn main() {
    match my_home_hue::config::parse_config(Path::new("./data/config.toml")) {
        Ok(config) => {
            let bridge = my_home_hue::hue::bridge::BridgeBuilder::new()
                            .ip_address(config.bridge_ip_address)
                            .username(config.username)
                            .build();

            match bridge.get_light(1) {
                Ok(body) => println!("{:#?}", body),
                Err(error) => println!("{:?}", error),
            }
        },
        Err(error) => println!("{:?}", error),
    }
}