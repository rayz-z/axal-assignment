use serde::Serialize;
use serde_json;
use std::io::Write;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use axal_rust::{fetch_tvl, calculate_tvl_change};

#[derive(Serialize)]
pub struct TVLMessage {
    tvl: u128,
    warning: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stored_tvl: u128 = 0;

    loop {
        let (tvl, last_tvl) = fetch_tvl().await?;

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

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tvl_drop_detection() {
        let stored = 100;
        let new = 70;
        let (change, warning) = calculate_tvl_change(stored, new);
        assert_eq!(change, 30);
        assert!(warning);

        let new2 = 90;
        let (_, warning2) = calculate_tvl_change(stored, new2);
        assert!(!warning2);
    }
}
