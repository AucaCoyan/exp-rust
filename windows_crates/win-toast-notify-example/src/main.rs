use win_toast_notify::WinToastNotify;

fn main() {
    WinToastNotify::new()
        .set_title("Espanso")
        .set_messages(vec!["Message 1", "Message 2"])
        .show()
        .expect("unable to show notification");

    println!("Finished!");
}
