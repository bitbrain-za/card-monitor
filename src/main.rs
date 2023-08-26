mod config;
use rumqttc::Event;
use rumqttc::{Client, MqttOptions, Outgoing, QoS};
use std::thread;
use std::time::Duration;
use usb_rfid_decoder as decoder;

fn main() {
    const CARD: [u16; 11] = [
        0x27, 0x27, 0x27, 0x1f, 0x22, 0x20, 0x27, 0x24, 0x25, 0x22, 0x28,
    ];
    let result = decoder::decode(&CARD);
    println!("{:?}", result.unwrap());

    let conf = match config::load("config.json") {
        Ok(conf) => conf,
        Err(e) => {
            println!("{}", e);
            let conf = config::Config::default();
            conf.save("config.json").unwrap();
            conf
        }
    };
    println!("{}", conf);

    let mut mqttoptions = MqttOptions::new("rumqtt-sync", conf.broker, conf.port);
    mqttoptions.set_credentials(conf.username, conf.password);
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    let (mut client, mut connection) = Client::new(mqttoptions, 10);

    let sender = thread::spawn(move || {
        let message = format!("Hello world!").as_bytes().to_vec();
        match client.publish(&conf.topic, QoS::AtLeastOnce, false, message) {
            Ok(_) => {
                for notification in connection.iter() {
                    if let Ok(Event::Outgoing(Outgoing::Publish(_))) = notification {
                        break;
                    }
                }
            }
            Err(e) => println!("Error publishing message: {:?}", e),
        }
    });

    let _done = sender.join();
}
