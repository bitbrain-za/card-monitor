use simple_logger::SimpleLogger;
mod monitor;
use monitor::config;
use monitor::homeassistant;
use std::env;

#[tokio::main]
async fn main() {
    SimpleLogger::new().init().unwrap();
    let args: Vec<String> = env::args().collect();
    log::debug!("{:?}", args);

    let config_path = match args.get(2) {
        Some(p) => p,
        None => "config.json",
    };

    let conf = match config::load(config_path) {
        Ok(conf) => conf,
        Err(e) => {
            println!("{}", e);
            let conf = config::Config::default();
            conf.save(config_path).expect("Error saving config");
            conf
        }
    };
    log::info!("Config Loaded: {}", conf);

    let disco = homeassistant::Discovery::card_monitor(&conf);
    log::info!("Discovery: {}", disco);

    let monitor = monitor::Monitor::new(&conf);
    monitor.run().await;
}
