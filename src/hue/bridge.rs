use std::marker::PhantomData;
use std::default::Default;
use serde::Serialize;
use crate::hue::light::Light;
use crate::hue::light::LightState;

/// Structure for Hue Bridge.
pub struct Bridge {
    /// IP Address of the bridge on your network.
    ip_address: String,
    /// An authorized username that  the bridge creates for you.
    username: String,
}

impl Bridge {
    /// Gets state of a given light.
    pub fn get_light(self, id: u32) -> Result<Light, failure::Error> {
        let url = format!("http://{}/api/{}/lights/{}", self.ip_address, self.username, id);
        let mut body: serde_json::Value = reqwest::get(&url)?.json()?;
        let light_state: LightState = serde_json::from_value(body["state"].take())?;
        Ok(Light::new(id, light_state))
    }

    /// Allows the user to change attributes of a schedule.
    pub fn set_schedule<T: Serialize + ?Sized>(self, id: u32, body: &T) -> Result<reqwest::Response,reqwest::Error> {
        let url = format!("http://{}/api/{}/schedules/{}", self.ip_address, self.username, id);
        reqwest::Client::new()
                        .put(&url)
                        .json(body)
                        .send()
    }
}

pub struct Empty;
pub struct Fully;
pub struct BridgeBuilder<IpAddress, Username> {
    ip_address: String,
    username: String,
    state: (PhantomData<IpAddress>, PhantomData<Username>),
}
impl BridgeBuilder<Empty, Empty> {
    pub fn new() -> Self {
        BridgeBuilder { 
            ip_address: Default::default(), 
            username: Default::default(),
            state: (PhantomData, PhantomData) 
        }
    }
}
impl BridgeBuilder<Fully, Fully> {
    pub fn build(self) -> Bridge {
        Bridge { 
            ip_address: self.ip_address,
            username: self.username,
        }
    }
}
impl<Username> BridgeBuilder<Empty, Username> {
    pub fn ip_address<S: Into<String>>(self, ip_address: S) -> BridgeBuilder<Fully, Username> {
        BridgeBuilder {
            ip_address: ip_address.into(),
            username: self.username,
            state: (PhantomData, self.state.1),
        }
    }
}
impl<IpAddress> BridgeBuilder<IpAddress, Empty> {
    pub fn username<S: Into<String>>(self, username: S) -> BridgeBuilder<IpAddress, Fully> {
        BridgeBuilder {
            ip_address: self.ip_address,
            username: username.into(),
            state: (self.state.0, PhantomData),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::hue::bridge::BridgeBuilder;

    #[test]
    #[ignore]
    fn test_get_light() {
        let bridge = BridgeBuilder::new()
                            .ip_address("192.168.1.10")
                            .username("3AyHHXYqfsEaWTD102MLlDNeBiJkbuk6XY8YOqK1")
                            .build();

        match bridge.get_light(1) {
            Ok(light) => assert!(light.is_on()),
            Err(err) => panic!("Error has occurred in test_get_light(): {:?}", err),
        }
    }

    #[test]
    #[ignore]
    fn test_set_schedule() {
        let bridge = BridgeBuilder::new()
                        .ip_address("192.168.1.10")
                        .username("3AyHHXYqfsEaWTD102MLlDNeBiJkbuk6XY8YOqK1")
                        .build();

        let body: HashMap<&str, &str> = [("status", "disabled")].iter().cloned().collect();
        match bridge.set_schedule(6, &body) {
            Ok(res) => assert!(res.status().is_success()),
            Err(err) => panic!("Error has occurred in test_set_schedule(): {:?}", err),
        }
    }
}