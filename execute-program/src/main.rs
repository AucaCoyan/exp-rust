//use execute::Execute;
use std::process::Command;

// const ICUE_PATH: &str = "C:\\Program Files\\Corsair\\CORSAIR iCUE 4 Software\\iCUE.exe";
const EXECUTE: &str = "&";
// const PYTHON_TEST: &str = "pipenv";

fn main() {
    let mut my_command = Command::new(EXECUTE);
    // my_command.arg(r#"--version"#);
    // let path = "'C:\\Program Files\\Corsair\\CORSAIR iCUE 4 Software\\iCUE.exe'";

    my_command.arg("'C:\\Program Files\\Corsair\\CORSAIR iCUE 4 Software\\iCUE.exe'");

    // execution
    match my_command.output() {
        Ok(o) => unsafe {
            println!("output: {}", String::from_utf8_unchecked(o.stdout));
        },
        Err(e) => {
            println!("There was an error! \n {}", e)
        }
    }
}
