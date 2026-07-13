use std::io::{self, Read};

pub fn cuantos_valen_7() {

    let mut entrada = String::new();
    io::stdin().read_to_string(&mut entrada).expect("Error al leer datos");
    let mut tokens = entrada.split_whitespace();

    let a:i32 = tokens.next().expect("Error al obtener datos").parse().expect("Error al convertir dato");
    let b:i32 = tokens.next().expect("Error al obtener datos").parse().expect("Error al convertir dato");
    let mut i:i32 = 0;

    if a == 7 {
        i += 1;
    } 
    
    if b == 7 {
        i += 1;
    }

    println!("{}",i);
}