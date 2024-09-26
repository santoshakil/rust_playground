extern crate fsevent;

use std::sync::mpsc::channel;
use std::thread;

#[cfg(not(target_os = "macos"))]
fn main() {}

#[cfg(target_os = "macos")]
fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        println!("Usage: cargo run -- <path>");
        return;
    }

    let path = args[1].clone();

    let (sender, receiver) = channel();

    let _t = thread::spawn(move || {
        let fsevent = fsevent::FsEvent::new(vec![path]);
        fsevent.observe(sender);
    });

    loop {
        let val = receiver.recv();
        // ignore .DS_Store
        if let Ok(val) = val {
            if val.path.ends_with(".DS_Store") {
                continue;
            }
            println!("{:?}", val);
        }
    }
}
