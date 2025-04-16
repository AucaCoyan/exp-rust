//! `module_path` gets you the name of the current rust module

use browser::brave::name;

mod browser;

fn main() {
    println!("{}", name());
    println!("Hello, world!");
}
