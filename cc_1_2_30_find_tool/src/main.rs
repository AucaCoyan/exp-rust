use cc;
fn main() {
    let tool = cc::windows_registry::find_tool("msvc", "devenv")
        .expect("unable to locate MSVC compiler, did you install Visual Studio?");
    println!("Hello, world!");
}
