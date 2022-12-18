use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

// constants
const SERVER_ADDRESS: &str = "127.0.0.1:1234";

fn main() {
    // starting
    println!("starting the server on {}", SERVER_ADDRESS);

    // listener
    let listener = TcpListener::bind(SERVER_ADDRESS).unwrap();

    // start
    println!("server listening {}", SERVER_ADDRESS);

    for stream in listener.incoming() {
        // let stream = stream.unwrap();

        println!("connection established!");
        // println!("stream: {:?}", &stream.ok());
        handle_connection(stream.unwrap())
    }
}

fn handle_connection(mut stream: TcpStream) {
    // read the buffer
    let mut buffer = [0; 1024];
    let len = stream.read(&mut buffer).unwrap();
    let message = String::from_utf8_lossy(&buffer[..len]);
    println!("recieved {}", message);

    // write the message
    stream.write_all(message.as_bytes());
}
