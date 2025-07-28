use cc;
fn main() {
    // let tool = cc::windows_registry::find_tool("msvc", "devenv")
    //     .expect("unable to locate MSVC compiler, did you install Visual Studio?");

    let cc_build = cc::Build::new();
    let tool_used = cc_build.get_compiler();
    println!("using {tool_used:#?}");
}
