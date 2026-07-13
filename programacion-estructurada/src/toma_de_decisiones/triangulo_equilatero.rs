use std::io::{self, Read};

pub fn triangulo_equilatero() {
    let mut entrada = String::new();
    io::stdin()
        .read_to_string(&mut entrada)
        .expect("Error al leer dato");

    let mut tokens = entrada.split_whitespace();

    let a: i32 = tokens
        .next()
        .expect("Error al obtener dato")
        .parse()
        .expect("Error al convertir dato");
    let b: i32 = tokens
        .next()
        .expect("Error al obtener dato")
        .parse()
        .expect("Error al convertir dato");
    let c: i32 = tokens
        .next()
        .expect("Error al obtener dato")
        .parse()
        .expect("Error al convertir dato");
    let d: i32 = tokens
        .next()
        .expect("Error al obtener dato")
        .parse()
        .expect("Error al convertir dato");

    if a == b && b == c {
        println!("1");
    } else if b == c && c == d {
        println!("1");
    } else if c == d && d == a {
        println!("1");
    } else if a == b && b == d {
        println!("1");
    } else {
        println!("0");
    }
}
