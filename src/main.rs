mod monitor;
use monitor::config;
use monitor::homeassistant;
use std::env;
use log::LevelFilter;
use systemd_journal_logger::JournalLog;

#[tokio::main]
async fn main() {
    JournalLog::default().install().unwrap();
    log::set_max_level(LevelFilter::Debug);
    let args: Vec<String> = env::args().collect();
    log::debug!("{:?}", args);

    let config_path = match args.get(1) {
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
