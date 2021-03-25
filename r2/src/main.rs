use std::io::{self, prelude:: *};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("lecture de stdin");
    let mut mot = input.split_whitespace();
    let r1 = mot.next().expect("1ere valeur");
    let s = mot.next().expect("2e valeur");

    let r1:i32=r1.parse().expect("Entier");
    let s:i32=s.parse().expect("Entier");


    let r2 = s*2 - r1;
    println!("{}", r2);
}
