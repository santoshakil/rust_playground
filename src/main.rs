use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let data = Arc::new(RwLock::new(Vec::new()));

    let handles: Vec<_> = (0..4)
        .map(|i| {
            let data = Arc::clone(&data);
            thread::spawn(move || {
                let mut data = data.write().unwrap();
                data.push(i);
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    let final_data = data.read().unwrap();
    println!("Final Vec: {:?}", *final_data);
}
