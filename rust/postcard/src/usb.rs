use std::time::Duration;

use nusb::transfer::{Bulk, In, Out};

use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
    env_logger::init();
    let di = nusb::list_devices()
        .await
        .unwrap()
        .find(|d| d.vendor_id() == 0xc0de && d.product_id() == 0xcafe)
        .expect("device should be connected");

    println!("Device info: {di:?}");

    let device = di.open().await.unwrap();

    let main_interface = device.claim_interface(0).await.unwrap();

    let mut writer = main_interface
        .endpoint::<Bulk, Out>(0x01)
        .unwrap()
        .writer(128)
        .with_num_transfers(8);

    let mut reader = main_interface
        .endpoint::<Bulk, In>(0x81)
        .unwrap()
        .reader(128)
        .with_num_transfers(8);

    println!("write");
    writer.write_all(b"hello world").await.unwrap();
    writer.flush().await.unwrap();
    writer.flush_end_async().await.unwrap();

    println!("read..");
    let mut buf = [0; 64];
    reader.read(&mut buf).await.unwrap();
    println!("read: {:?}", buf);
}
