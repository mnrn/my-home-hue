
use std::marker::PhantomData;

pub struct Bridge {
    ip_address: String,
    username: String,
}

impl Bridge {
    pub fn get_light(self, id: i32) -> Result<serde_json::Value, reqwest::Error> {
        let url = format!("http://{}/api/{}/lights/{}", self.ip_address, self.username, id);
        let body = reqwest::get(&url)?
                            .json()?;
        Ok(body)
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