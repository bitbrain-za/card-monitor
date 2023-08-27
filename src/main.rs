use simple_logger::SimpleLogger;
use usb_rfid_decoder as decoder;
mod monitor;
use monitor::config;
use monitor::homeassistant;

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

    let disco = homeassistant::Discovery::card_monitor(&conf);
    log::info!("Discovery: {}", disco);

    let monitor = monitor::Monitor::new(&conf);
    monitor.run().await;
}
