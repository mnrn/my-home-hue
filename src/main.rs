extern crate failure;
extern crate serde_derive;
extern crate serde_json;
extern crate toml;
extern crate reqwest;
extern crate log;
extern crate env_logger;
extern crate chrono;

use std::collections::HashMap;
use std::io::Write;
use serde_json::Value;
use log::{LevelFilter, error, info};
use env_logger::Builder;
use chrono::Local;
use my_home_hue::hue::bridge::BridgeBuilder;

fn main() {
    static BRIDGE_IP_ADDRESS: &'static str = "192.168.1.10";
    static USERNAME: &'static str = "3AyHHXYqfsEaWTD102MLlDNeBiJkbuk6XY8YOqK1";
    const LIGHT_ID: u32 = 1;
    const SCHEDULE_ID: u32 = 1;

    // Calls Builder::format to set a closure which formats each message text with timestamp.
    Builder::new()
        .format(|buf, record| {
            writeln!(buf, "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(), record.args())
        })
        .filter(None, LevelFilter::Info)
        .init();

    // Gets state of a my home hue light.
    let bridge = BridgeBuilder::new()
                            .ip_address(BRIDGE_IP_ADDRESS)
                            .username(USERNAME)
                            .build();
    let light = bridge.get_light(LIGHT_ID)
                    .map_err(|err| error!("Error has occurred in Bridge::get_light: {:?}", err))
                    .unwrap();
    info!("{}", format!("Light/{} is {}", LIGHT_ID, if light.is_on() { "On" } else { "Off" }));

    // Change status of a schedule.
    let schedule_status = if light.is_on() { "enabled" } else { "disabled" };
    let body: HashMap<&str, &str> = [("status", schedule_status)].iter().cloned().collect();
    let res: Value = bridge.set_schedule(SCHEDULE_ID, &body)
                        .map_err(|err| error!("Error has occurred in Bridge::set_schedule: {:?}", err))
                        .unwrap()
                        .json()
                        .map_err(|err| error!("Error has occurred in reqwest::Response::json: {:?}", err))
                        .unwrap();
    info!("{:#?}", res);
}