use std::io;

pub fn conversiones_temperaturas() {
    let mut e = String::new();
    io::stdin().read_line(&mut e).expect("error al leer");
    let c: i32 = e.trim().parse().expect("escribe numero");

    let k = c + 273;
    let f = ((c * 9) / 5) + 32;
    let r = (c * 4) / 5;

    println!("{} {} {}", k, f, r);
}
