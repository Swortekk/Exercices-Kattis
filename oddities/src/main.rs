use std::io::{self, prelude:: *};


/* fn is_odd(n : i32) --> bool {
    return nù2!=0;}


fn main() {
   let mut input = String::new();
   io::stdin().read_to_string(&mut input).expect("lecture de stdin");
   let mut lines = input.lines();
   lines.next();
   for line in lines {
       let n = line.parse::<i32> println!("");().expect("entier");
       if is_odd(n){
           println!("{} is odd", n);
       }
       else {
        println!("{} is even", n);
       }
   }

}

fn abracadabra(n : i32){

    for i in 1..=n {
    println!("{} Abracadabra", i)
    }
    
} println!("");
    io::stdin().read_to_string(&mut input).expect("lecture de stdin");
    let lines = input.lines();
    for line in lines {
        let n = line.parse::<i32>().expect("entier");
        abracadabra(n);
    }
}

fn fizzbuzz(x: i32, y:i32,n:i32){
    for i in 1..n+1{
       

        if (i % x==0){
            print!("Fizz");
        }
        
        if (i % y==0){
            print!("Buzz");
        }
        if ( i% x!=0 && i % y !=0){
            print!("{}",i);
        }
        println!("");
    }
    
   
}

fn main(){
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("lecture de stdin");
    let mut my_iterator =input.split_whitespace();

    let x = my_iterator.next().expect("Entier");
    let x = x.parse::<i32>().expect("Entier");

    let y= my_iterator.next().expect("Entier");
    let y = y.parse::<i32>().expect("Entier");

    let n = my_iterator.next().expect("Entier");
    let n = n.parse::<i32>().expect("Entier");
    fizzbuzz(x,y,n);
   
}




fn main(){
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("lecture de stdin");
    let x = input;
    
    let mut vector = x.chars().collect::<Vec<char>>();
    println!("{:?}", vector);
    vector.reverse();
    println!("{:?}", vector);
    let mut a = vector.iter().collect::<String>();
    println!("{}", a);

}
*/

fn main(){
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("lecture de stdin");
    let mut  yo = input;
    assert_eq!(split.as_str(),yo);
    split.next();
        
    
}