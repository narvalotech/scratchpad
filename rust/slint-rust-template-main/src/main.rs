slint::include_modules!();

fn main() {
    let main_window = AppWindow::new().unwrap();

    main_window.run().unwrap();
}