use std::io;

fn main() {

    println!("Type a number: ");

    let mut number = String::new();

    io::stdin().read_line(&mut number).expect("Not a Number");

    println!("number: {}", number);

}
