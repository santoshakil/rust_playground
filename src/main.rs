use fs_extra::dir::{self, CopyOptions};
use std::env;
use std::fs;

fn main() {
    if env::args().count() < 3 {
        println!("Usage: <src> <dst>");
        std::process::exit(1);
    }

    let src = env::args().nth(1).unwrap();
    let dst = env::args().nth(2).unwrap();

    // Remove the destination directory if it exists
    if std::path::Path::new(&dst).exists() {
        if let Err(err) = fs::remove_dir_all(&dst) {
            eprintln!("Failed to remove existing directory {}: {}", dst, err);
            return;
        }
    }

    let config = CopyOptions::new().content_only(true).overwrite(true);

    if let Err(err) = dir::copy(&src, &dst, &config) {
        eprintln!("Error: {err}");
    }
}
