use nats;
use std::{thread, time};

use clap::{ArgEnum, Parser};

const MSG: &str = r#"{"action":null,"additional_information":null,"box_id":"e5bd05e7-8972-4850-81a6-204a26ee4ca3","category":null,"create_time":"2022-05-23T21:04:13.013361","event_id":1070002,"justification":null,"host_id":"e5bd05e7-8972-4850-81a6-204a26ee4ca3","message":"test_message","protocol":null,"receive_time":"2022-05-24T08:04:25.330940","request_id":null,"severity":"low","source_ip":null,"source_mac":null,"source_id":null,"source_hostname":null,"source_port":null,"target_ip":null,"target_mac":null,"target_id":null,"target_hostname":null,"target_port":null,"user":null}"#;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Messages per second
    #[clap(short, long)]
    rate: Option<f32>,
}
fn main() -> () {
    let cli = Cli::parse();
    let sleep_time: Option<time::Duration> = cli
        .rate
        .and_then(|rate: f32| Some(time::Duration::from_secs_f32(1.0 / rate)));
    let nc = nats::connect("127.0.0.1:4222").unwrap();
    loop {
        nc.publish("msg.test", MSG).unwrap();
        if let Some(s) = sleep_time {
            thread::sleep(s)
        }
    }
}
