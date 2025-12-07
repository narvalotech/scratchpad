// #[derive(Debug)]
// pub enum KeyEvent {
//     Unknown,
//     KeyDown,
//     KeyUp,
//     KeyLeft,
//     KeyRight,
//     KeySelect,
//     KeyBack,
//     KeyPower,
// }

// #[derive(Debug)]
// pub struct InputEvent {
//     pub key: KeyEvent,
// }

use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to a peer
    let mut stream = TcpStream::connect("127.0.0.1:9999").await?;

    // Write some data.
    stream.write_all(b"hello world!\n").await?;

    Ok(())
}
