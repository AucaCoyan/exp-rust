#![allow(unused)]
use core::fmt;
use inpyt::input;
use std::io::stdin;

#[derive(Debug)]
struct ClientList {
    list: Vec<Client>,
}

impl ClientList {
    fn new() -> Self {
        ClientList { list: vec![] }
    }

    fn add(&mut self, client: Client) {
        self.list.push(client);
    }

    fn total(&self) -> usize {
        self.list.len()
    }

    fn client_id_exists(id: usize, list: &ClientList) -> Option<usize> {
        let iter = list.list.iter().enumerate();
        for (key, client) in iter {
            if client.id == id {
                return Some(key);
            }
        }
        return None;
    }
}

impl fmt::Display for ClientList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(for client in &self.list {
            writeln!(f, "{client}");
        })
    }
}

struct ProductList {
    list: Vec<Product>,
}

impl ProductList {
    fn new() -> Self {
        ProductList { list: vec![] }
    }

    fn add(&mut self, product: Product) {
        self.list.push(product);
    }

    fn total(&self) -> usize {
        self.list.len()
    }
}

impl fmt::Display for ProductList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(for product in &self.list {
            writeln!(f, "{product}");
        })
    }
}

#[derive(Debug)]
struct Client {
    name: String,
    id: usize,
}

impl Client {
    fn new(name: String, id: usize) -> Self {
        Self { name: name, id: id }
    }
}

impl fmt::Display for Client {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = &self.name;
        let id = &self.id;
        writeln!(f, "Client {id} - {name},\n")
    }
}

struct Product {
    name: String,
    id: usize,
}

impl Product {
    fn new(name: String, id: usize) -> Self {
        Self { name: name, id: id }
    }
}

impl fmt::Display for Product {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = &self.name;
        let id = &self.id;
        writeln!(f, "Product {id} - {name},\n")
    }
}

fn opcion_1(ref_to_client_list: &mut ClientList) {
    let name_client = input!("Ingrese el nombre del cliente: ");
    let next_id = ref_to_client_list.total() + 1;
    // create the new client
    let new_client = Client::new(name_client, next_id);
    // add the client to the client list
    ref_to_client_list.add(new_client);
}

fn opcion_2(ref_to_product_list: &mut ProductList) {
    let name_product = input!("Ingrese el nombre del producto: ");
    let next_id = ref_to_product_list.total() + 1;
    // create the new client
    let new_product = Product::new(name_product, next_id);
    // add the client to the client list
    ref_to_product_list.add(new_product);
}

fn opcion_3(ref_to_client_list: &ClientList) {
    let search_id = input!("Inserte el ID a buscar: ")
        .trim_end()
        .parse::<usize>()
        .unwrap();
    let result = ClientList::client_id_exists(search_id, ref_to_client_list);
    if result.is_some() {
        let index = result.unwrap();
        println!("client found!");
        println!("{}", ref_to_client_list.list[index]);
    }
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

fn opcion_7(ref_to_client_list: &ClientList) {
    println!("{ref_to_client_list}")
}

fn opcion_8(ref_to_product_list: &ProductList) {
    println!("{ref_to_product_list}")
}

fn main() {
    // lista de clientes
    let mut client_list = ClientList::new();
    let mut product_list = ProductList::new();

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
            Ok(1) => opcion_1(&mut client_list),  // 1-Crear cliente
            Ok(2) => opcion_2(&mut product_list), // 2-Crear producto
            Ok(3) => opcion_3(&mut client_list),  // 3-Consultar cliente por ID
            Ok(4) => opcion_4(),                  // 4-Buscar cliente por nombre
            Ok(5) => opcion_5(),                  // 5-Borrar cliente por nombre
            Ok(6) => opcion_6(),                  // 6-Borrar producto
            Ok(7) => opcion_7(&client_list),      // 7-Imprimir lista de clientes
            Ok(8) => opcion_8(&product_list),     // 8-Imprimir lista de productos
            Ok(0) => {
                println!("goodbye!");
                std::process::exit(0)
            }
            _ => panic!(),
        };
    }
}
