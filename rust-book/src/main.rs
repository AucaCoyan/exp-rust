#![allow(unused)]

use std::str::FromStr;

struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
#[derive(Debug)]
struct WriteMessage {
    string: String,
    id: u128,
} // full struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

#[derive(Debug)]
enum MessageType {
    QuitMsg,
    MoveMsg,
    ChatMsg { content: String, id: u128 },
    ChangeColorMsg,
}

#[derive(Debug)]
struct Message {
    _type: MessageType,
}

enum Option {
    None,
    Some,
}

impl Message {
    fn new(mess_type: String) -> Self {
        Self {
            _type: MessageType::ChatMsg {
                content: mess_type,
                id: 1,
            },
        }
    }
}
fn main() {
    let var = Option::Some;

    let content = String::from("Mi contenido de mensaje");
    let my_mes = Message::new(String::from("Mi Mensaje"));

    println!("Mi mensaje: {:?}", my_mes)
}
