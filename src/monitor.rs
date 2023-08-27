use std::default::Default;
pub mod config;
use rumqttc::{AsyncClient, MqttOptions, QoS};
use std::fmt;
use std::io;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tokio::task;
use tokio::time::sleep;
pub mod homeassistant;

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

        self.notify(&client, &"start1".to_string()).await;

        while !self.stop.load(Ordering::Relaxed) {
            sleep(std::time::Duration::from_millis(500)).await;
            let mut user_input = String::new();
            let stdin = io::stdin();
            let _ = stdin.read_line(&mut user_input);

            if user_input.trim() == "stop" {
                self.stop.store(true, Ordering::Relaxed);
            } else {
                self.notify(&client, &user_input).await;
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

    async fn notify(&self, client: &AsyncClient, message: &String) {
        let topic = format!("{}/{}", self.config.mqtt_config.topic, self.config.id);
        Monitor::publish(client, &topic, message).await;
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
