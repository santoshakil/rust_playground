use std::sync::{Mutex, OnceLock};

static ONCE: OnceLock<Mutex<i64>> = OnceLock::new();

fn main() {
    let lock = ONCE.get_or_init(|| Mutex::new(42));
    let mut value = lock.lock().unwrap();
    *value = 50;
    println!("Value: {}", *value);
}
