use nats;
use std::error::Error;
use tokio;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let nc = nats::asynk::connect("127.0.0.1:4222").await?;
    println!("connected");
    let sub = nc.subscribe("msg.test").await?;

    // Receive a message.
    while let Some(msg) = sub.next().await {
        println!("{:#?}", msg)
    }
    Ok(())
}
