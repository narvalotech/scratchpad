use bluer::{
    gatt::local::{
        Application, Characteristic, CharacteristicControlEvent, CharacteristicNotify,
        CharacteristicNotifyMethod, Service,
    },
    AdapterEvent, Address,
};
use futures::StreamExt;
use std::time::Duration;
use tokio::{io::AsyncWriteExt, sync::mpsc, time::sleep};
use uuid::Uuid;

// GAP Role: Central
const TARGET_DEVICE_ADDRESS: &str = "XX:XX:XX:XX:XX:XX"; 

// GATT Role: Server
const BATTERY_SERVICE_UUID: Uuid = Uuid::from_u128(0x0000180f_0000_1000_8000_00805f9b34fb);
const BATTERY_LEVEL_CHAR_UUID: Uuid = Uuid::from_u128(0x00002a19_0000_1000_8000_00805f9b34fb);

#[tokio::main]
async fn main() -> bluer::Result<()> {
    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    adapter.set_powered(true).await?;

    println!("Central/Server active on {}", adapter.name());

    let (sub_tx, mut sub_rx) = mpsc::channel(1);

    let characteristic = Characteristic {
        uuid: BATTERY_LEVEL_CHAR_UUID,
        notify: Some(CharacteristicNotify {
            notify: true,
            method: CharacteristicNotifyMethod::Io(Box::new(move |mut writer, mut control: bluer::gatt::local::CharacteristicControl| {
                let sub_tx = sub_tx.clone();
                Box::pin(async move {
                    println!("[GATT] Peer subscribed to Battery Level.");
                    let _ = sub_tx.send(writer).await;
                    
                    // Explicitly handle the control stream events
                    while let Some(event) = control.next().await {
                        match event {
                            CharacteristicControlEvent::NotifyStopped => break,
                            _ => (),
                        }
                    }
                    println!("[GATT] Peer unsubscribed.");
                })
            })),
            ..Default::default()
        }),
        ..Default::default()
    };

    let service = Service {
        uuid: BATTERY_SERVICE_UUID,
        primary: true,
        characteristics: vec![characteristic],
        ..Default::default()
    };

    let app = Application {
        services: vec![service],
        ..Default::default()
    };

    let _app_handle = adapter.serve_gatt_application(app).await?;

    // GAP Central: Connect to peer
    let target_addr: Address = TARGET_DEVICE_ADDRESS.parse().unwrap();
    println!("[GAP] Scanning for {}...", target_addr);
    
    let mut scan_events = adapter.discover_devices().await?;
    while let Some(event) = scan_events.next().await {
        if let AdapterEvent::DeviceAdded(addr) = event {
            if addr == target_addr { break; }
        }
    }

    let device = adapter.device(target_addr)?;
    device.connect().await?;
    println!("[GAP] Connected. Waiting for peer to subscribe to our GATT Server...");

    // Wait for subscription and send 10 notifications
    if let Some(mut writer) = sub_rx.recv().await {
        for i in 1..=10 {
            let val = 100 - (i * 5);
            println!("-> Notifying: {}%", val);
            if let Err(_) = writer.write_all(&[val as u8]).await {
                println!("Write failed; peer likely disconnected.");
                break;
            }
            sleep(Duration::from_secs(1)).await;
        }
    }

    println!("[GAP] Task complete. Disconnecting.");
    let _ = device.disconnect().await;

    Ok(())
}
