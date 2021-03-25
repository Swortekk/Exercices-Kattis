use std::io::{self, prelude::*};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("erreur");
    let ligne = input.lines().next().expect("1ere ligne");
    let mut mot = ligne.split_whitespace();
    mot.next();
    let mot2= mot.next().expect("2e mot");
    let nb_mot:i32=mot2.parse().expect("Entier");
    println!("{}", nb_mot);
}
