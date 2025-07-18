use std::{fs, path::PathBuf};

fn main() {
    let target_link_dir = PathBuf::from("/usr/local/bin");
    // let exec_path = std::env::current_exe().expect("Unable to get the current .exe");
    let target_link_path = target_link_dir.join("espanso");
    println!("{target_link_path:?}");
    if target_link_path.exists() && target_link_path.is_symlink() {
        println!("It exists and is a symlink!");

        let result = fs::remove_file(target_link_path.clone());
        println!("{result:?}");
    }
    if target_link_path.exists() {
        println!("It still exists though");
    } else {
        println!("It was removed!");
    }
    println!("Done!");
}
