

use std::io;
use std::io::Read;


fn shorten (mots: &str) -> String {
 let mut result = String::new();
 for m in mots.split("-") {
 result.push(m.chars().next().expect("Premier caractère"));
 }
 return result;


}
fn main() {
 let mut input = String::new();
 io::stdin().read_to_string(&mut input);
 let mots = input.lines().next().expect("Première ligne");
 println!("{}", shorten(mots));
} 
