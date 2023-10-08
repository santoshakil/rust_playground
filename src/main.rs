use std::sync::{OnceLock, RwLock};

static ONCE: OnceLock<RwLock<i64>> = OnceLock::new();

fn main() {
    let lock = ONCE.get_or_init(|| RwLock::new(0));
    {
        let mut value = lock.write().unwrap();
        *value = 1;
    }
    println!("Value: {}", lock.read().unwrap());
}
