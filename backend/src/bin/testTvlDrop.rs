use serde::Serialize;
use serde_json;
use std::io::Write;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use axal_rust::{calculate_tvl_change};

#[derive(Serialize)]
pub struct TVLMessage {
    tvl: u128,
    warning: bool,
}

fn test_fetch_tvl() -> (u128, u128) {
    (60_000_000_000_000, 60_000_000_000_000)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stored_tvl: u128 = 0;

    for idx in 0..4 {
        let (mut tvl, last_tvl) = test_fetch_tvl();

        if idx == 3 {
            tvl = 20_000_000_000_000;
        }

        // convert into USD
        let conv_tvl = tvl / 1_000_000;
        let conv_last_tvl = last_tvl / 1_000_000;

        let (tvl_change, warning) = calculate_tvl_change(stored_tvl, conv_tvl);
        stored_tvl = conv_tvl;

        println!("TVL: {}", tvl);
        println!("TVL USD: {}", conv_tvl);
        println!("Change: {}", tvl_change);
        println!("Warning: {}", warning);

        // send over TCP
        let msg = TVLMessage { tvl: stored_tvl, warning };
        let json_msg = serde_json::to_string(&msg)?;
        let mut stream = TcpStream::connect("127.0.0.1:4000")?;
        stream.write_all(json_msg.as_bytes())?;
        stream.write_all(b"\n")?;

        thread::sleep(Duration::from_secs(1));
    }

    Ok(())
}