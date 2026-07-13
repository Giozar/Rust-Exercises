use std::io::{self, Read};
use std::f32::consts::PI;

pub fn prgramando_formulas_raras() {

    let mut entrada = String::new();
    
    io::stdin().read_to_string(&mut entrada).expect("Error al leer");

    let mut tokens = entrada.split_whitespace();

    let x: f32 = tokens.next().expect("Falta X").parse().expect("Error en X");
    let y: f32 = tokens.next().expect("Falta Y").parse().expect("Error en Y");
    let z: f32 = tokens.next().expect("Falta Z").parse().expect("Error en Z");

    let r = (x + x * (y + z.powi(2))) / ((x + PI) * (y + PI));

    println!("{}", r);
}