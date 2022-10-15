use std::io::stdin;

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

fn main() -> Result<()> {
    println!("Hello, there!  What is your name?");

    let mut buffer = String::new();

    // `read_line` returns `Result` of bytes read
    stdin().read_line(&mut buffer)?;
    let res = match buffer.trim_end() {
        "" => "Ah, well I can respect your wish to remain anonymous.".into(),
        name => format!("Hello, {name}.  Nice to meet you!"),
    };
    println!("{res}");

    Ok(())
}
