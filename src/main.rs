use notify_rust::Notification;

fn main() {
    Notification::new()
        .summary("Firefox News")
        .body("This will almost look like a real firefox notification.")
        .icon("firefox")
        .show()
        .unwrap();
}
