use std::marker::PhantomData;
use serde_json::Value;
use crate::hue::light::Light;
use crate::hue::light::LightState;

pub struct Bridge {
    ip_address: String,
    username: String,
}

impl Bridge {
    /// Gets state of a given light.
    pub fn get_light(self, id: i32) -> Result<Light, failure::Error> {
        let url = format!("http://{}/api/{}/lights/{}", self.ip_address, self.username, id);
        let mut body: Value = reqwest::get(&url)?.json()?;
        let light_state: LightState = serde_json::from_value(body["state"].take())?;
        Ok(Light::new(id, light_state))
    }
}

pub struct Empty;
pub struct Fully;
pub struct BridgeBuilder<IpAddress, Username> {
    ip_address: Option<String>,
    username: Option<String>,
    state: (PhantomData<IpAddress>, PhantomData<Username>),
}
impl BridgeBuilder<Empty, Empty> {
    pub fn new() -> Self {
        BridgeBuilder { 
            ip_address: None, 
            username: None,
            state: (PhantomData, PhantomData) 
        }
    }
}
impl BridgeBuilder<Fully, Fully> {
    pub fn build(self) -> Bridge {
        Bridge { 
            ip_address: self.ip_address.unwrap(),
            username: self.username.unwrap(),
        }
    }
}
impl<Username> BridgeBuilder<Empty, Username> {
    pub fn ip_address<T: Into<String>>(self, ip_address: T) -> BridgeBuilder<Fully, Username> {
        BridgeBuilder {
            ip_address: Some(ip_address.into()),
            username: self.username,
            state: (PhantomData, self.state.1),
        }
    }
}
impl<IpAddress> BridgeBuilder<IpAddress, Empty> {
    pub fn username<T: Into<String>>(self, username: T) -> BridgeBuilder<IpAddress, Fully> {
        BridgeBuilder {
            ip_address: self.ip_address,
            username: Some(username.into()),
            state: (self.state.0, PhantomData),
        }
    }
}