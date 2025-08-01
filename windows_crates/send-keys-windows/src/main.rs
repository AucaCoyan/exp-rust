use windows::{
    core::w,
    Win32::UI::Input::KeyboardAndMouse::{
        SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP,
        VK_LWIN,
    },
};

/// with the help of <https://gist.github.com/littletsu/d1c1b512d6843071144b7b89109a8de2>
/// Windows API: SendInput
/// https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendinput
/// `windows` crate
/// https://microsoft.github.io/windows-docs-rs/doc/windows/
/// windows crate mdbook
/// https://kennykerr.ca/rust-getting-started/index.html
/// feature flags search
/// https://microsoft.github.io/windows-rs/features/#/0.59.0/search/sendinput
///
fn main() {
    println!("begin");
    let my_widestring = w!("some string");
    println!("{my_widestring}");

    let something = &"auca";
    send_string(something);

    println!("end!");
}

fn send_string(string: &str) {
    let input = [
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VK_LWIN,
                    wScan: 78,
                    dwFlags: KEYBD_EVENT_FLAGS(0),
                    time: 0, // The time stamp for the event, in milliseconds. If this parameter is zero, the system will provide its own time stamp.
                    dwExtraInfo: 0,
                },
            },
        },
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VK_LWIN,
                    wScan: 78,
                    dwFlags: KEYEVENTF_KEYUP,
                    time: 0, // The time stamp for the event, in milliseconds. If this parameter is zero, the system will provide its own time stamp.
                    dwExtraInfo: 0,
                },
            },
        },
    ];

    let ipsize = std::mem::size_of::<INPUT>() as i32;
    unsafe {
        SendInput(&input, ipsize);
    }
}
