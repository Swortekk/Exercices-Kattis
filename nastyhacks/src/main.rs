use std::io::{self, prelude::*};

fn main() {
    let mut entree = String::new();
    io::stdin().read_to_string(&mut entree).expect("lecture données");
    let mut line = entree.lines();
    line.next();
    for l in line
    {
        let mut mot = l.split_whitespace();
        let i = mot.next().expect("i").parse::<i32>().expect("Entier");
        let e = mot.next().expect("e").parse::<i32>().expect("Entier");
        let c = mot.next().expect("c").parse::<i32>().expect("Entier");
        decide(i,e,c);

    }
}

fn decide(r:i32,e:i32,c:i32) {
if e -c > r
{
        println!("advertise");
}
else if e -c < r
{
        println!("do not advertise");
}
else
{
        println!("does not matter");
}

}