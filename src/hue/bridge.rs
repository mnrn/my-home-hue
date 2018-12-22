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

pub struct BridgeBuilderIpAddress;

impl BridgeBuilderIpAddress {
    pub fn new() -> Self {
        BridgeBuilderIpAddress
    }
    pub fn ip_address(self, ip_address: String) -> BridgeBuilderUsername {
        BridgeBuilderUsername { ip_address: ip_address }
    }
}

pub struct BridgeBuilderUsername {
    ip_address: String,
}

impl BridgeBuilderUsername {
    pub fn username(self, username: String) -> BridgeBuilder {
        BridgeBuilder { 
            ip_address: self.ip_address,
            username: username
        }
    }
}

pub struct BridgeBuilder {
    ip_address: String,
    username: String,
}

impl BridgeBuilder {
    pub fn new() -> BridgeBuilderIpAddress { 
        BridgeBuilderIpAddress 
    }
    pub fn build(self) -> Bridge {
        Bridge { 
            ip_address: self.ip_address,
            username: self.username,
        }
    }
}