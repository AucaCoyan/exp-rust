use std::io;
use std::path::Path;
use winreg::enums::*;
use winreg::RegKey;

fn main() -> io::Result<()> {
    println!("Reading some system info...");

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let http_key = hkcu.open_subkey(
        "SOFTWARE\\Microsoft\\Windows\\Shell\\Associations\\UrlAssociations\\http\\UserChoice",
    )?;
    //let cur_ver = hklm.open_subkey("SOFTWARE\\Classes\\http\\shell\\open\\command\\(Default)")?;
    let pf: String = http_key.get_value("ProgId")?;
    println!("{pf}");

    println!("And now lets write something...");
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = Path::new("Software").join("WinregRsExample1");
    let (key, disp) = hkcu.create_subkey(&path)?;

    match disp {
        REG_CREATED_NEW_KEY => println!("A new key has been created"),
        REG_OPENED_EXISTING_KEY => println!("An existing key has been opened"),
    }

    key.set_value("TestSZ", &"written by Rust")?;
    let sz_val: String = key.get_value("TestSZ")?;
    //key.delete_value("TestSZ")?;
    println!("TestSZ = {}", sz_val);
    //
    //key.set_value("TestMultiSZ", &vec!["written", "by", "Rust"])?;
    //let multi_sz_val: Vec<String> = key.get_value("TestMultiSZ")?;
    //key.delete_value("TestMultiSZ")?;
    //println!("TestMultiSZ = {:?}", multi_sz_val);
    //
    //key.set_value("TestDWORD", &1234567890u32)?;
    //let dword_val: u32 = key.get_value("TestDWORD")?;
    //println!("TestDWORD = {}", dword_val);
    //
    //key.set_value("TestQWORD", &1234567891011121314u64)?;
    //let qword_val: u64 = key.get_value("TestQWORD")?;
    //println!("TestQWORD = {}", qword_val);
    //
    //key.create_subkey("sub\\key")?;
    //hkcu.delete_subkey_all(&path)?;
    //
    //println!("Trying to open nonexistent key...");
    //hkcu.open_subkey(&path).unwrap_or_else(|e| match e.kind() {
    //    io::ErrorKind::NotFound => panic!("Key doesn't exist"),
    //    io::ErrorKind::PermissionDenied => panic!("Access denied"),
    //    _ => panic!("{:?}", e),
    //});
    Ok(())
}
