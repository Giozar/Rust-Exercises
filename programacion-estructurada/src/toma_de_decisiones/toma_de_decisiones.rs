use std::io::{self, Read};

pub fn toma_de_decisiones() {
    let mut entrada = String::new();
    io::stdin().read_to_string(&mut entrada).expect("Error al leer entrada");

    let mut tokens = entrada.split_whitespace();

    let mut a: i128 = tokens.next().expect("Error al obtener a").parse().expect("error al covertir a");
    let mut b: i128 = tokens.next().expect("Error al obtener a").parse().expect("error al convertir b");

    if a + b == 5 {
        b = 3 + b;
        println!("{}",2*a+b);
    } else {
        a = a-1;
        if (7*a + b)%2 == 0 {
            println!("{}", a-b);
        } else {
            println!("{}", a*b);
        }
    }
}