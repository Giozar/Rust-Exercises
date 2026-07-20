use std::io::{self,Read};

pub fn el_perrito_que_quiere_hueso() {
    let mut entrada = String::new();
    io::stdin().read_to_string(&mut entrada).expect("Error al leer dato");

    let mut tokens = entrada.split_whitespace();

    let l1: i32 = tokens.next().expect("No se puedo obtener valor").parse().expect("Error al convertir");
    let t1: i32 = tokens.next().expect("No se puedo obtener valor").parse().expect("Error al convertir");
    let l2: i32 = tokens.next().expect("No se puedo obtener valor").parse().expect("Error al convertir");
    let t2: i32 = tokens.next().expect("No se puedo obtener valor").parse().expect("Error al convertir");

    if l1 > l2 && t1 > t2 {
        println!("Hueso 1");
    } else if l2 > l1 && t2 > t1 {
        println!("Hueso 2");
    } else {
        println!("Perrito confundido :(");
    }
}
