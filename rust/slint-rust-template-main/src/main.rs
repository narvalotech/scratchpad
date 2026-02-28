mod host_types;
use crate::host_types::{Address};

slint::include_modules!();
use slint::{ModelRc, SharedString, StandardListViewItem, VecModel};
use std::rc::Rc;

#[derive(Clone)]
struct Device {
    address: Address,
    rssi: i32,
    name: String,
    data: String,
    _private_field: u32,
}

// A helper function to map your internal domain logic to the UI representation
fn create_row_from_device(device: &Device) -> ModelRc<StandardListViewItem> {
    let addr = format!("{}", device.address);
    let row_data = vec![
        StandardListViewItem::from(SharedString::from(&addr)),
        StandardListViewItem::from(SharedString::from(device.rssi.to_string())),
        StandardListViewItem::from(SharedString::from(&device.name)),
        StandardListViewItem::from(SharedString::from(&device.data)),
    ];

    // Wrap the Vec into a Model and then into the ModelRc expected by the outer table
    ModelRc::from(Rc::new(VecModel::from(row_data)))
}

fn main() {
    let ui = AppWindow::new().unwrap();

    let ui_handle = ui.as_weak();
    let devices_: Box<Vec<Device>> = Box::new(Vec::new());
    let devices = Box::leak(devices_);

    std::thread::spawn(move || {
        loop {
            std::thread::sleep(std::time::Duration::from_millis(500));
            println!("Create device");

            let new_device = Device {
                address: Address::new(1, 0x00aA7DDA7113),
                rssi: -65,
                name: "Kitchen Sensor".into(),
                data: "0x010203".into(),
                _private_field: 42,
            };
            devices.push(new_device);

            let devs = devices.clone();
            ui_handle.upgrade_in_event_loop(move |ui| {
                let devices_rows = Rc::new(VecModel::<ModelRc<StandardListViewItem>>::default());
                ui.set_scan_results(ModelRc::from(devices_rows.clone()));

                let devices_rows_copy = devices_rows.clone();
                for device in devs {
                    let row = create_row_from_device(&device);
                    devices_rows_copy.push(row);
                }
            }).unwrap();
        }
    });
    
    ui.run().unwrap();
}
