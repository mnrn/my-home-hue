use std::collections::HashMap;
use crate::hue::bridge::Bridge;

pub struct Service {
    bridge: Bridge,
}

impl Service {
    pub fn new(bridge: Bridge) -> Service {
        Service { bridge: bridge, }
    }

    pub fn exec(&self) -> Result<(), failure::Error> {
        let light = self.bridge.get_light(1)?;
        let body: HashMap<&str, &str> = if light.is_on() {
            [("status", "enabled")]
        } else {
            [("status", "disabled")]
        }.iter().cloned().collect();
        
        let res = self.bridge.set_schedule(1, &body)?;
        println!("{:#?}", res);
        Ok(())
    }
}