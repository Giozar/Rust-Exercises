use std::io::{self, Read};

pub fn descuento_en_comics() {
    let mut entrada = String::new();

    io::stdin().read_to_string(&mut entrada).expect("Error a leer entrada");

    let mut tokens = entrada.split_whitespace();

    let mut c1:f32 = tokens.next().expect("Error al obtener c1").parse().expect("Error al convertir c1"); 
    let mut c2:f32 = tokens.next().expect("Error al obtener c2").parse().expect("Error al convertir c2");


    if c1 + c2 >= 100.0 {
        println!("Habra descuento inicial para ambos");

        let c1o = c1;
        let c2o = c2;

        c1 = c1 - (c1*0.10);
        c2 = c2 - (c2*0.10);

        if c1  > c2 {
            println!("Ademas, habra descuento especial para el primer comprador");
            println!("{:.2} {:.2}", c1 - (c1o*0.10), c2 );

        } else if c2 > c1 {
            println!("Ademas, habra descuento especial para el segundo comprador");
            println!("{:.2} {:.2}", c1, c2 - (c2o*0.10) );
            
        } else {
            println!("{:.2} {:.2}", c1, c2 );
        }

    } else {
        println!("No habra descuento :(");
        println!("{:.2} {:.2}", c1, c2);
    }

}