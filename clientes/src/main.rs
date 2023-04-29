#![allow(unused)]
use inpyt::input;
use std::io::stdin;

#[derive(Debug)]
struct Cliente {
    name: String,
    id: u128,
}

impl Cliente {
    fn new(name: String, id: u128) -> Self {
        Self { name: name, id: 1 }
    }
}

#[derive(Debug)]
struct ListaClientes {
    vec: Vec<Cliente>,
}

impl ListaClientes {
    fn new() -> Self {
        Self { vec: Vec::new() }
    }

    fn add(&mut self, cliente: Cliente) {
        self.vec.push(cliente)
    }

    fn count_clients() {
        todo!()
    }
}

fn opcion_1() {
    println!("elegiste 1!");
    let name_cliente = input!("Ingrese el nombre del cliente");
    Cliente::new(name_cliente, 2);
}

fn opcion_2() {
    println!("elegiste 2!");
    todo!()
}

fn opcion_3() {
    println!("elegiste 3!");
    todo!()
}

fn opcion_4() {
    println!("elegiste 4!");
    todo!()
}

fn opcion_5() {
    println!("elegiste 5!");
    todo!()
}

fn opcion_6() {
    println!("elegiste 6!");
    todo!()
}

fn opcion_7() {
    println!("elegiste 7!");
    todo!()
}
fn opcion_8() {
    println!("elegiste 8!");
    todo!()
}
fn main() {
    // lista de clientes
    let mut lista_cliente = ListaClientes::new();

    // cliente harcodeado
    let nombre_cliente1 = String::from("Auca");
    let id_cliente1 = 1;

    let cliente1: Cliente = Cliente {
        name: nombre_cliente1,
        id: id_cliente1,
    };

    lista_cliente.add(cliente1);

    println!("{:?}", lista_cliente);

    // main loop
    println!("Bienvenido al programa de clientes");
    let texto_inicio = "
    Elija una opcion:
    1-Crear cliente
    2-Crear producto
    3-Consultar cliente por ID
    4-Buscar cliente por nombre/apellido
    5-Borrar cliente por nombre/apellido/ID
    6-Borrar producto
    7-Imprimir lista de clientes
    8-Imprimir lista de productos

    Para finalizar, presione 0 ";

    loop {
        println!("{}", texto_inicio);

        let mut response = String::new();

        stdin().read_line(&mut response).expect("Failed input");

        match response.trim_end().parse::<u8>() {
            Ok(1) => opcion_1(), // 1-Crear cliente
            Ok(2) => opcion_2(), // 2-Crear producto
            Ok(3) => opcion_3(), // 3-Consultar cliente por ID
            Ok(4) => opcion_4(), // 4-Buscar cliente por nombre/ap
            Ok(5) => opcion_5(), // 5-Borrar cliente por nombre/ap
            Ok(6) => opcion_6(), // 6-Borrar producto
            Ok(7) => opcion_7(), // 7-Imprimir lista de clientes
            Ok(8) => opcion_8(), // 8-Imprimir lista de productos
            Ok(0) => {
                println!("goodbye!");
                std::process::exit(0)
            }
            _ => panic!(),
        };

        println!("You've just picked {}", response);
    }
}
