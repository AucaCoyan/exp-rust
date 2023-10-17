#![allow(unused)]

use fastrand::usize;

/// Tipos de cliente que puede tener. De mayor a menor, A a D
#[derive(Debug)]
enum Cliente {
    A,
    B,
    C,
    D,
}

/// Estados que la caja puede tener
enum Caja {
    Busy,
    Free,
}

impl Caja {
    fn new() -> Self {
        Caja::Free
    }
}

fn main() {
    let available_clients = [Cliente::A, Cliente::B, Cliente::C, Cliente::D];

    let new_client: &Cliente = &available_clients[usize(..available_clients.len())];
    println!("Generated client: {:#?}", new_client);
}
