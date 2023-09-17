use tokio::task;

#[tokio::main]
async fn main() {
    let x = "Hello, world!";
    for _ in 0..5 {
        _ = task::spawn(async move {
            println!("Async start!");
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            println!("Async end! - {}", x);
        });
    }
    tokio::time::sleep(std::time::Duration::from_secs(20)).await;
    println!("Done! - {}", x);
}
