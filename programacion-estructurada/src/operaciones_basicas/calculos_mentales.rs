use std::io;

pub fn calculos_mentales() {
    let mut e = String::new();
    io::stdin().read_line(&mut e).expect("error al leer");
    let r: f64 = e.trim().parse().expect("escribe el numero de entrada");

    let r1 = ((r + 5.0) * (r + 5.0) / (r / 3.0))
        * ((r + 5.0) * (r + 5.0) / (r / 3.0))
        * ((r + 5.0) * (r + 5.0) / (r / 3.0));

    println!(
        "{} {} {} {} {}",
        r,
        r + 5.0,
        (r + 5.0) * (r + 5.0),
        (r + 5.0) * (r + 5.0) / (r / 3.0),
        r1
    );
}
