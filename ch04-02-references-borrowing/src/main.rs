fn main() {
    let s1 = String::from("Hello");

    // In its definition, we take &String rather than String.
    // These ampersands represent references,
    // and they allow you to refer to some value without taking ownership of it.
    // [reference: https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html]
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // Mutable References
    let mut s = String::from("Hello, World");

    change(&mut s);

    println!("s = {}", s);

    // Mutable references have one big restriction:
    // you can have only one mutable reference to a particular piece of data at a time.

    let mut s2 = String::from("hello");

    // first mutable borrow occurs here
    let r1 = &mut s2;
    let r2 = &mut s2;

    println!("{}, {}", r1, r2);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(str: &mut String) {
    str.push_str("+PLUS!!");
}