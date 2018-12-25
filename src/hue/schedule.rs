use serde_derive::Deserialize;

/// Command to execute when the scheduled event occurs.
#[derive(Debug, Deserialize), allow(dead_code)]
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
#[derive(Debug, Deserialize), allow(dead_code)]
pub struct Schedule {
    /// The name of the schedule.
    name: Option<String>,
    /// Description of the schedule.
    description: Option<String>,
    /// Command to execute when the scheduled event occurs.
    command: ScheduleCommand,
    /// Time when the scheduled event will occur.
    localtime: String,
    /// Time when the scheduled event will occur.
    /// DEPRECATED Will be removed in the future. 
    time: String,
    /// Document was not found in https://www.developers.meethue.com/
    created: Option<String>,
    /// "enabled" Schedule is enabled
    /// "disabled" Schedule is disabled by user.
    status: Option<String>,
    /// When true: Resource is automatically deleted when not referenced anymore in any resource link. 
    /// Only on creation of resource. “false” when omitted.
    recycle: Option<bool>,
    /// If set to true, the schedule will be removed automatically if expired,
    /// if set to false it will be disabled. Default is true
    autodelete: Option<bool>,
}