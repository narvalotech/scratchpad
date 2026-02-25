slint::include_modules!();
use slint::{ModelRc, SharedString, StandardListViewItem, VecModel};
use std::rc::Rc;

struct Device {
    address: String,
    rssi: i32,
    name: String,
    data: String,
    _private_field: u32,
}

// A helper function to map your internal domain logic to the UI representation
fn create_row_from_device(device: &Device) -> ModelRc<StandardListViewItem> {
    let row_data = vec![
        StandardListViewItem::from(SharedString::from(&device.address)),
        StandardListViewItem::from(SharedString::from(device.rssi.to_string())),
        StandardListViewItem::from(SharedString::from(&device.name)),
        StandardListViewItem::from(SharedString::from(&device.data)),
    ];

    // Wrap the Vec into a Model and then into the ModelRc expected by the outer table
    ModelRc::from(Rc::new(VecModel::from(row_data)))
}

fn main() {
    let ui = AppWindow::new().unwrap();
    let devices_rows = Rc::new(VecModel::<ModelRc<StandardListViewItem>>::default());

    ui.set_scan_results(ModelRc::from(devices_rows.clone()));

    let devices_rows_copy = devices_rows.clone();
    ui.on_start_scan(move || {
        // In a real app, this might come from a Bluetooth crate
        let new_device = Device {
            address: "00:1A:7D:DA:71:13".into(),
            rssi: -65,
            name: "Kitchen Sensor".into(),
            data: "0x010203".into(),
            _private_field: 42,
        };

        // Use our nice refactored function
        let row = create_row_from_device(&new_device);
        devices_rows_copy.push(row);
    });

    ui.run().unwrap();
}
