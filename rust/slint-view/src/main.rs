use std::env;
use std::path::PathBuf;
use slint_interpreter::{Compiler, ComponentHandle, live_preview::LiveReloadingComponent};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run -- <path_to_file.slint>");
        std::process::exit(1);
    }

    let file_path = PathBuf::from(&args[1]).canonicalize()?;

    println!("Watching for changes in: {:?}", file_path);

    let compiler = Compiler::default();
    let result = LiveReloadingComponent::new(compiler, file_path, "AppWindow".to_string());

    if let Ok(c) = result {
        c.borrow().instance().show().unwrap();
        slint_interpreter::run_event_loop().unwrap();
    }

    Ok(())
}
