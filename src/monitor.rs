use std::default::Default;
pub mod config;
use rumqttc::{AsyncClient, MqttOptions, QoS};
use std::fmt;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tokio::task;
pub mod homeassistant;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use usb_rfid_decoder as decoder;

pub struct Monitor {
    config: config::Config,
    mqtt_options: MqttOptions,
    stop: AtomicBool,
}

impl Monitor {
    pub fn new(config: &config::Config) -> Monitor {
        let mut mqtt_options = MqttOptions::new(
            &config.id,
            &config.mqtt_config.broker,
            config.mqtt_config.port,
        );
        mqtt_options.set_credentials(&config.mqtt_config.username, &config.mqtt_config.password);
        mqtt_options.set_keep_alive(Duration::from_secs(5));

        log::info!(
            "Connecting to broker: {}:{}",
            config.mqtt_config.broker,
            config.mqtt_config.port
        );

        Monitor {
            config: config.clone(),
            mqtt_options,
            stop: AtomicBool::new(false),
        }
    }

    pub async fn run(&self) {
        let (client, mut connection) = AsyncClient::new(self.mqtt_options.clone(), 1);

        task::spawn(async move {
            while let Ok(notification) = connection.poll().await {
                log::debug!("Received notification: {:?}", notification);
            }
        });

        let disco_message = homeassistant::Discovery::card_monitor(&self.config);
        Monitor::publish(&client, &disco_message.topic(), &disco_message.to_string()).await;
        let file = File::open(&self.config.device_config.path)
            .unwrap_or_else(|_| panic!("Unable to open {}", self.config.device_config.path));
        let mut reader = BufReader::new(file);

        let mut buf = [0u8; 512];
        while !self.stop.load(Ordering::Relaxed) {
            let n = match reader.read(&mut buf) {
                Ok(n) => n,
                _ => 0,
            };

            if n != 0 {
                for (i, c) in buf.iter().enumerate().take(n) {
                    log::debug!("{} - {:#x}", i, c);
                }
            }
            if n >= 8 {
                if let Ok(result) = decoder::decode(&buf) {
                    log::debug!("Decoded Card: {:?}", result);
                    self.notify(&client, &result).await;
                }
            }
        }
    }

    async fn publish(client: &AsyncClient, topic: &String, message: &String) {
        log::debug!("Publishing message: {} to {}", message, topic);

        if let Err(e) = client
            .publish(topic, QoS::AtLeastOnce, false, message.as_bytes().to_vec())
            .await
        {
            log::error!("Error publishing message: {:?}", e);
        }
    }

    async fn notify(&self, client: &AsyncClient, message: &str) {
        let topic = format!("{}/{}/value", self.config.mqtt_config.topic, self.config.id);
        let message = format!("{{\"card\": {}}}", message.trim());
        Monitor::publish(client, &topic, &message).await;
    }
}

impl Default for Monitor {
    fn default() -> Self {
        let config = config::Config::default();
        Monitor::new(&config)
    }
}

impl fmt::Display for Monitor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Monitor {{ mqtt: {}, device: {} }}",
            self.config.mqtt_config, self.config.device_config
        )
    }
}
