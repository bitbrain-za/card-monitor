use std::default::Default;
pub mod config;
use rumqttc::{AsyncClient, Client, MqttOptions, Outgoing, QoS};
use std::fmt;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tokio::task;
use tokio::time::sleep;

pub struct Monitor {
    config: config::Config,
    path: String,
    mqtt_options: MqttOptions,
    stop: AtomicBool,
}

impl Monitor {
    pub fn new(config: &config::Config, path: String) -> Monitor {
        let mut mqtt_options = MqttOptions::new(&config.id, &config.broker, config.port);
        mqtt_options.set_credentials(&config.username, &config.password);
        mqtt_options.set_keep_alive(Duration::from_secs(5));

        log::info!("Connecting to broker: {}:{}", config.broker, config.port);

        Monitor {
            config: config.clone(),
            path,
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

        self.notify(&client, &"start".to_string()).await;
        while !self.stop.load(Ordering::Relaxed) {
            sleep(std::time::Duration::from_secs(5)).await;
        }
    }

    async fn publish(client: &AsyncClient, topic: &String, message: &String) {
        log::debug!("Publishing message: {}", message);

        match client
            .publish(topic, QoS::AtLeastOnce, false, message.as_bytes().to_vec())
            .await
        {
            Err(e) => log::error!("Error publishing message: {:?}", e),
            _ => {}
        }
    }

    async fn notify(&self, client: &AsyncClient, message: &String) {
        let topic = &self.config.topic;
        Monitor::publish(&client, topic, message).await;
    }
}

impl Default for Monitor {
    fn default() -> Self {
        let config = config::Config::default();
        let file = "/dev/hidraw0".to_string();
        Monitor::new(&config, file)
    }
}

impl fmt::Display for Monitor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Monitor {{ config: {}, file: {} }}",
            self.config, self.path
        )
    }
}
