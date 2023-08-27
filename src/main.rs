use rumqttc::Event;
use rumqttc::{Client, MqttOptions, Outgoing, QoS};
use simple_logger::SimpleLogger;
use std::thread;
use std::time::Duration;
use usb_rfid_decoder as decoder;
mod monitor;
use monitor::config;

#[tokio::main]
async fn main() {
    SimpleLogger::new().init().unwrap();

    const CARD: [u16; 11] = [
        0x27, 0x27, 0x27, 0x1f, 0x22, 0x20, 0x27, 0x24, 0x25, 0x22, 0x28,
    ];
    let result = decoder::decode(&CARD);
    log::debug!("{:?}", result.unwrap());

    let conf = match config::load("config.json") {
        Ok(conf) => conf,
        Err(e) => {
            println!("{}", e);
            let conf = config::Config::default();
            conf.save("config.json").unwrap();
            conf
        }
    };
    log::info!("Config Loaded: {}", conf);

    let monitor = monitor::Monitor::new(&conf, "dummy".to_string());

    monitor.run().await;
}
