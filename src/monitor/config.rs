use std::default::Default;
use std::fmt;
use std::fs::File;

#[derive(Clone, Default, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub mqtt_config: MqttConfig,
    pub device_config: DeviceConfig,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct MqttConfig {
    pub id: String,
    pub broker: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub topic: String,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DeviceConfig {
    pub path: String,
}

pub fn load(path: &str) -> Result<Config, String> {
    let file = File::open(path).map_err(|e| format!("Failed to open config file: {}", e))?;
    let config =
        serde_json::from_reader(file).map_err(|e| format!("Failed to parse config file: {}", e))?;
    Ok(config)
}

impl Config {
    pub fn save(&self, path: &str) -> Result<(), String> {
        let file =
            File::create(path).map_err(|e| format!("Failed to create config file: {}", e))?;
        serde_json::to_writer_pretty(file, self)
            .map_err(|e| format!("Failed to write config file: {}", e))?;
        Ok(())
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Config {{ MQTT: {{ broker: {}, port: {}, username: {}, password: {} }}, Device: {{ path: {} }}",
            self.mqtt_config.broker, self.mqtt_config.port, self.mqtt_config.username, self.mqtt_config.password, self.device_config.path
        )
    }
}

impl fmt::Display for MqttConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "MqttConfig {{ broker: {}, port: {}, username: {}, password: {} }}",
            self.broker, self.port, self.username, self.password
        )
    }
}

impl fmt::Display for DeviceConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DeviceConfig {{ path: {} }}", self.path)
    }
}

impl Default for MqttConfig {
    fn default() -> Self {
        MqttConfig {
            id: "card-monitor".to_string(),
            broker: "localhost".to_string(),
            port: 1883,
            username: "username".to_string(),
            password: "password".to_string(),
            topic: "bitbrain/rfid".to_string(),
        }
    }
}

impl Default for DeviceConfig {
    fn default() -> Self {
        DeviceConfig {
            path: "/dev/hidraw0".to_string(),
        }
    }
}
