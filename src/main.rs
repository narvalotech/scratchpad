#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum KeyEvent {
    Unknown,
    KeyDown,
    KeyUp,
    KeyLeft,
    KeyRight,
    KeySelect,
    KeyBack,
    KeyPower,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct InputEvent {
    pub key: KeyEvent,
}

use serde::{Serialize, Deserialize};
use postcard::{from_bytes, to_allocvec};

use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let output: Vec<u8> = to_allocvec(&InputEvent {
        key: KeyEvent::KeyUp,
    }).unwrap();

    // Connect to a peer
    let mut stream = TcpStream::connect("127.0.0.1:9999").await?;

    stream.write_all(&output).await?;

    let decoded: InputEvent = from_bytes(&output).unwrap();
    println!("{:?}", decoded);

    Ok(())
}
