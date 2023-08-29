use crate::config::Config;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Discovery {
    pub device: Device,
    pub name: String,
    pub state_topic: String,
    pub unique_id: String,
    pub value_template: String,
    pub enabled_by_default: bool,
    pub icon: String,
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
            device: Device {
                identifiers: vec![format!("{}", config.id)],
                manufacturer: "Bitbrain".to_string(),
                model: "Bitbrain rfid sensor".to_string(),
                name: config.friendly_name.clone(),
                sw_version: "3.1.0".to_string(),
            },
            name: "RFID".to_string(),
            state_topic: format!("{}/value", config.base_topic()),
            unique_id: config.id.clone(),
            value_template: "{{value_json.card}}".to_string(),
            enabled_by_default: true,
            icon: "mdi:card-bulleted-outline".to_string(),
        }
    }

    pub fn topic(&self) -> String {
        format!("homeassistant/sensor/{}/rfid/config", self.unique_id)
    }
}

impl fmt::Display for Discovery {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = serde_json::to_string_pretty(&self).unwrap();
        write!(f, "{}", s)
    }
}
