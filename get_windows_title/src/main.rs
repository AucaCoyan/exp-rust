use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowTextW};
fn main() {
    let title: &mut [u16] = &mut [];
    unsafe { GetWindowTextW(GetForegroundWindow(), title) };
    println!("Hello, world!");
    println!("{title:?}");
}
