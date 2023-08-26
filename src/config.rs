use std::default::Default;
use std::fmt;
use std::fs::File;

#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub broker: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub topic: String,
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
            "Config {{ broker: {}, port: {}, username: {}, password: {} }}",
            self.broker, self.port, self.username, self.password
        )
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            broker: "localhost".to_string(),
            port: 1883,
            username: "username".to_string(),
            password: "password".to_string(),
            topic: "bitbrain/rfid".to_string(),
        }
    }
}
