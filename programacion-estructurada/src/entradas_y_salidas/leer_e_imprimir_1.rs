use std::io;

pub fn leer_e_imprimir_1() {
    let mut entrada1 = String::new();
    io::stdin().read_line(&mut entrada1).expect("Error al leer entrada 1");
    let a : i32 = entrada1.trim().parse().expect("Error al convertir entrada1");
    
    let mut entrada2 = String::new();
    io::stdin().read_line(&mut entrada2).expect("Error al leer entrada 2");

    let b : i32 = entrada2.trim().parse().expect("Error al convertir entrada2");


    println!("{} {}", a, b);
}