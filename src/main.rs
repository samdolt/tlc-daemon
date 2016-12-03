#[macro_use]
extern crate log;
extern crate env_logger;
extern crate subcmd;
extern crate rumqtt;

use rumqtt::{MqttOptions, MqttClient, QoS};
use rumqtt::MqRequest;

use std::thread::sleep;
use std::time::Duration;

fn connect_to_broker() -> MqRequest {
    let options = MqttOptions::new()
        .set_keep_alive(5)
        .set_reconnect(3)
        .set_client_id("tlc-daemon")
        .broker("localhost:1883");

    let client = MqttClient::new(options).message_callback(move |message| {
        println!("message --> {:?}", message);
    });


    client.start().expect("Unable to start mqtt")
}


fn main() {
    env_logger::init().expect("Unable to init env_logger");
    trace!("Starting tlc-daemon");

    let mqtt = connect_to_broker();

    mqtt.subscribe(vec![("hello/world", QoS::Level0)]).expect("Subscribe");

    loop {
        let payload = "Hello World!".to_string();
        mqtt.publish("hello/world", QoS::Level0, payload.into_bytes()).expect("Publish");
        sleep(Duration::from_secs(3));
    }


}
