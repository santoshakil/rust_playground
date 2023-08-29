use sqlx::{sqlite::SqlitePoolOptions, Executor, Row};

const DB_URL: &str = "sqlite:hashes.db";

#[tokio::main]
async fn main() {
    _ = std::fs::File::create("hashes.db");
    let db = SqlitePoolOptions::new()
        .max_connections(10)
        .connect(DB_URL)
        .await;
    if db.is_err() {
        eprintln!("Error initializing DB.");
        return;
    }
    let db = db.unwrap();
    let conn = db.acquire().await;
    if conn.is_err() {
        eprintln!("Error acquiring DB connection.");
        return;
    }
    let mut conn = conn.unwrap();
    let _ = conn
        .execute(
            "CREATE TABLE IF NOT EXISTS hashes (
                id INTEGER PRIMARY KEY,
                hash TEXT NOT NULL,
                salt TEXT NOT NULL,
                password TEXT NOT NULL
            )",
        )
        .await;
    let _ = conn
        .execute("CREATE INDEX IF NOT EXISTS hash_index ON hashes (hash)")
        .await;

    let start = std::time::Instant::now();
    let count = sqlx::query("SELECT COUNT(*) FROM hashes")
        .fetch_one(&mut *conn)
        .await
        .unwrap()
        .get::<i64, _>(0);
    println!("Counted {} rows in {:?}", count, start.elapsed());
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;

    if count == 0 {
        let start = std::time::Instant::now();
        let tx = db.begin().await;
        if tx.is_err() {
            eprintln!("Error beginning transaction.");
            return;
        }
        let tx = tx.unwrap();
        for i in 0..100000 {
            let start = std::time::Instant::now();
            _ = sqlx::query(&"INSERT INTO hashes (hash, salt, password) VALUES (?, ?, ?)")
                .bind(format!("hash{}", i))
                .bind(format!("salt{}", i))
                .bind(format!("password{}", i))
                .execute(&mut *conn)
                .await;
            println!("Write: {:?}", start.elapsed());
        }
        _ = tx.commit().await;
        println!("Write Total: {:?}", start.elapsed());
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    }

    let start = std::time::Instant::now();
    for i in 0..100000 {
        let start = std::time::Instant::now();
        if let Ok(row) = sqlx::query(&"SELECT * FROM hashes WHERE hash = ?")
            .bind(format!("hash{}", i))
            .fetch_one(&mut *conn)
            .await
        {
            let hash = row.get::<String, _>("hash");
            println!("Read {:?}: {:?}", hash, start.elapsed());
        } else {
            println!("Not found: {:?}", start.elapsed());
        }
    }
    println!("Read Total: {:?}", start.elapsed());
}
