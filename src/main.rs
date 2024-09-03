fn main() {
    if std::env::args().count() < 3 {
        println!("Usage: <src> <dst>");
        std::process::exit(1);
    }

    let src = std::env::args().nth(1).unwrap();
    let dst = std::env::args().nth(2).unwrap();

    // Remove the destination directory if it exists
    if std::path::Path::new(&dst).exists() {
        if let Err(err) = std::fs::remove_dir_all(&dst) {
            eprintln!("Error: {err}");
        }
    }

    let config = fs_extra::dir::CopyOptions::new()
        .content_only(true)
        .overwrite(true);

    if let Err(err) = fs_extra::dir::copy(&src, &dst, &config) {
        eprintln!("Error: {err}");
    }
}
