extern crate my_home_hue;

use std::collections::HashMap;
use my_home_hue::config::ConfigBuilder;
use my_home_hue::hue::bridge::BridgeBuilder;

#[test]
#[ignore]
fn test_sync_light_state_with_schedule() {
    let config = ConfigBuilder::new()
                                .config_path("./data/config.toml")
                                .build()
                                .expect("Error has occurred in configBuilder.build");
    let bridge = BridgeBuilder::new()
                                .ip_address(config.bridge_ip_address().clone())
                                .username(config.username().clone())
                                .build();

    let body: HashMap<&str, &str> = [("status", "enabled")].iter().cloned().collect();
    let res = bridge.set_schedule(6, &body)
                    .expect("Error has occurred in bridge.set_schedule");
    assert!(res.status().is_success());
}