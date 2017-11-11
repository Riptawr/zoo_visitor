#![deny(unused_mut)]
extern crate zookeeper;
#[macro_use]
extern crate log;
extern crate env_logger;

use std::time::Duration;
use std::env;
use zookeeper::{Watcher, WatchedEvent, ZooKeeper};

struct LoggingWatcher;
impl Watcher for LoggingWatcher {
    fn handle(&self, e: WatchedEvent) {
        info!("{:?}", e)
    }
}

fn zk_server_urls() -> String {
    let key = "ZOOKEEPER_SERVERS";
    match env::var(key) {
        Ok(val) => val,
        Err(_) => "localhost:2181".to_string(),
    }
}

fn get_kafka_broker(id: &str) {
    let zk_urls = zk_server_urls();
    // println!("connecting to {}", zk_urls);

    let zk = ZooKeeper::connect(&*zk_urls, Duration::from_secs(15), LoggingWatcher).unwrap();

    // zk.add_listener(|zk_state| println!("New ZkState is {:?}", zk_state));

    let get_data = zk.get_data(&format!("/brokers/ids/{}", id), true);
    
    let parsed = match get_data {
        Ok(data) => String::from_utf8(data.0).unwrap(),
        Err(e) => {format!("Error {:?}", e); std::process::exit(1)}
    };

    println!("{}", parsed);
}

fn main() {
    env_logger::init().unwrap();
    let args: Vec<String> = std::env::args().skip(1).collect();

    get_kafka_broker(&args[0]);
}