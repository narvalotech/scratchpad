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
use postcard::{from_bytes_cobs};

use tokio::net::TcpListener;
use tokio::io::AsyncReadExt;
use std::io;

async fn process_socket<T: AsyncReadExt + Unpin>(mut socket: T) {
    println!("OPENED");

    let mut v: Vec<u8> = Vec::new();
    loop {
        match socket.read_u8().await {
            Ok(byte) => {
                if byte != 0 {
                    v.push(byte);
                }
                println!("RX: {:x} VEC {:?}", byte, v);
                let decoded: Result<InputEvent, _> = from_bytes_cobs(&mut v.clone());
                if let Ok(data) = decoded {
                    println!("DECODED: {:?}", data);
                    v.clear();
                }
            },
            Err(_) => {
                println!("CLOSED");
                return;
            },
        }
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:9999").await?;

    println!("LISTEN");

    loop {
        let (socket, _) = listener.accept().await?;
        process_socket(socket).await;
    }
}
