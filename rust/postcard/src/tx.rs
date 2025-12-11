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
use postcard::{from_bytes_cobs, to_allocvec_cobs};

use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use std::error::Error;

async fn write_event_to_stream(stream: &mut TcpStream, event: InputEvent) {
    let output: Vec<u8> = to_allocvec_cobs(&event).unwrap();

    stream.write_all(&output).await.unwrap();

    let decoded: InputEvent = from_bytes_cobs(&mut output.clone()).unwrap();
    println!("Written: {:?}", decoded);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to a peer
    let mut stream = TcpStream::connect("127.0.0.1:9999").await?;

    let e1 = InputEvent {
        key: KeyEvent::KeyUp,
    };

    let e2 = InputEvent {
        key: KeyEvent::KeyDown,
    };

    write_event_to_stream(&mut stream, e1).await;
    write_event_to_stream(&mut stream, e2).await;

    Ok(())
}
