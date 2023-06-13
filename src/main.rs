use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Example with a normal bool
    let normal_bool = false;
    let normal_bool_arc = Arc::new(Mutex::new(normal_bool));

    let normal_bool_thread = thread::spawn(move || {
        for _ in 0..10 {
            let mut normal_bool = normal_bool_arc.lock().unwrap();
            if !*normal_bool {
                println!("Normal bool is false. Updating to true.");
                *normal_bool = true;
            } else {
                println!("Normal bool is true. Skipping update.");
            }
            thread::sleep(std::time::Duration::from_millis(100));
        }
    });
    normal_bool_thread.join().unwrap();

    // Example with an AtomicBool
    let atomic_bool = AtomicBool::new(false);
    let atomic_bool_arc = Arc::new(atomic_bool);

    let atomic_bool_thread = thread::spawn(move || {
        for _ in 0..10 {
            if !atomic_bool_arc.load(Ordering::SeqCst) {
                println!("AtomicBool is false. Updating to true.");
                atomic_bool_arc.store(true, Ordering::SeqCst);
            } else {
                println!("AtomicBool is true. Skipping update.");
            }
            thread::sleep(std::time::Duration::from_millis(100));
        }
    });
    atomic_bool_thread.join().unwrap();
}
