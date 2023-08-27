use crate::config::Config;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Discovery {
    pub availability: Vec<Availability>,
    pub device: Device,
    pub name: String,
    pub state_topic: String,
    pub unique_id: String,
    pub unit_of_measurement: String,
    pub value_template: String,
    pub device_class: String,
    pub enabled_by_default: bool,
    pub icon: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Availability {
    pub topic: String,
    pub value_template: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Device {
    pub identifiers: Vec<String>,
    pub manufacturer: String,
    pub model: String,
    pub name: String,
    pub sw_version: String,
}

impl Discovery {
    pub fn card_monitor(config: &Config) -> Self {
        Discovery {
            availability: vec![Availability {
                topic: format!("{}/availability", config.base_topic()),
                value_template: "{{value_json.available}}".to_string(),
            }],
            device: Device {
                identifiers: vec![format!("{}", config.id)],
                manufacturer: "bitbrain".to_string(),
                model: "rfid".to_string(),
                name: config.friendly_name.clone(),
                sw_version: "0.1.0".to_string(),
            },
            name: "bitbrain-rfid".to_string(),
            state_topic: config.base_topic(),
            unique_id: config.id.clone(),
            unit_of_measurement: "".to_string(),
            value_template: "{{value_json.card}}".to_string(),
            device_class: "none".to_string(),
            enabled_by_default: true,
            icon: "mdi:rfid".to_string(),
        }
    }
}

impl fmt::Display for Discovery {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = serde_json::to_string_pretty(&self).unwrap();
        write!(f, "{}", s)
    }
}
