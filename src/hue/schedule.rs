use serde_derive::Deserialize;

/// Command to execute when the scheduled event occurs.
#[derive(Debug, Deserialize)]
pub struct ScheduleCommand {
    /// Path to a light resource, a group resource or any other bridge resource.
    address: String,
    /// JSON value to be sent to the relevant resource.
    body: serde_json::Value,
    /// The HTTPS method used to send the body to the given address.
    /// Either “POST”, “PUT”, “DELETE” for local addresses.
    method: String,
}

/// a schedule that have been added to the bridge.
#[derive(Debug, Deserialize)]
pub struct Schedule {
    /// The name of the schedule.
    name: String,
    /// Description of the schedule.
    description: String,
    /// Command to execute when the scheduled event occurs.
    command: ScheduleCommand,
    /// Time when the scheduled event will occur.
    localtime: String,
    /// Time when the scheduled event will occur.
    /// DEPRECATED Will be removed in the future. 
    time: Option<String>,
    /// Document was not found in https://www.developers.meethue.com/
    created: String,
    /// "enabled" Schedule is enabled
    /// "disabled" Schedule is disabled by user.
    status: String,
    /// Document was not found in https://www.developers.meethue.com/
    recycle: bool,
    /// If set to true, the schedule will be removed automatically if expired,
    /// if set to false it will be disabled. Default is true
    autodelete: Option<bool>,
}