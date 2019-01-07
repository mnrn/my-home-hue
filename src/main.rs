extern crate failure;
extern crate serde_derive;
extern crate serde_json;
extern crate reqwest;
extern crate log;
extern crate env_logger;
extern crate chrono;

use std::collections::HashMap;
use std::io::Write;
use failure::Error;
use serde_json::Value;
use log::{LevelFilter, error, info};
use env_logger::Builder;
use chrono::Local;
use my_home_hue::hue::bridge::BridgeBuilder;

// My home hue service.
fn service() -> Result<(), Error> {
    // Gets state of a my home hue light.
    let bridge = BridgeBuilder::new()
                            .ip_address("192.168.1.10")
                            .username("3AyHHXYqfsEaWTD102MLlDNeBiJkbuk6XY8YOqK1")
                            .build();
    const LIGHT_ID: u32 = 1;
    let light = bridge.get_light(LIGHT_ID)?;
    info!("{}", format!("Light/{} is {}", LIGHT_ID, if light.is_on() { "On" } else { "Off" }));

    // Change status of a schedule.
    let schedule_status = if light.is_on() { "enabled" } else { "disabled" };
    let body: HashMap<&str, &str> = [("status", schedule_status)].iter().cloned().collect();
    const SCHEDULE_ID: u32 = 1;
    let res: Value = bridge.set_schedule(SCHEDULE_ID, &body)?
                        .json()?;
    info!("{:#}", res);
    Ok(())
}

fn main() {
    // Call Builder::format to set a closure which formats each message text with timestamp.
    Builder::new()
        .format(|buf, record| {
            writeln!(buf, "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(), record.args())
        })
        .filter(None, LevelFilter::Info)
        .init();

    // Execute my home hue service.
    if let Err(err) = service() {
        error!("Error has occurred while processing service: {}", err);
    }
}