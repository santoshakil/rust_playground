#[tokio::main]
async fn main() {
    let (s, r) = crossbeam_channel::unbounded::<String>();

    tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        s.send("Hello world!".to_string()).unwrap();
    });

    loop {
        if let Ok(msg) = r.try_recv() {
            println!("{}", msg);
            break;
        }
    }
}
