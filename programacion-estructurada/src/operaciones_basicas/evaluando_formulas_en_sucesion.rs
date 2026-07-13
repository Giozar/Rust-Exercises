use std::io::{self, Read};

pub fn evaluando_formulas_en_sucesion() {

    let mut entrada = String::new();

    io::stdin().read_to_string(&mut entrada).expect("Error al leer entrada");

    let mut tokens = entrada.split_whitespace();

    let x:f32 = tokens.next().expect("Se esperaba dato").parse().expect("Error al leer dato");

    let y = (x + 5.0)/(2.0*(x+1.0));

    let z = (y.powi(2) + x * (x-2.0*y))/(x*y);

    println!("{}", z);

}

