use std::io::{self, Read};

pub fn leer_e_imprimir_2() {
    let mut entrada = String::new();

    io::stdin().read_to_string(&mut entrada).expect("Error a leer entrada");

    let mut tokens = entrada.split_whitespace();

    let a:i128 = tokens.next().expect("Error al obtener a").parse().expect("Error al convertir a"); 
    let b:i128 = tokens.next().expect("Error al obtener b").parse().expect("Error al convertir b");

    println!("{} {}", a, b );

}