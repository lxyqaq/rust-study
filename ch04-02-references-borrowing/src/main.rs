fn main() {
    let s1 = String::from("Hello");

    // In its definition, we take &String rather than String.
    // These ampersands represent references,
    // and they allow you to refer to some value without taking ownership of it.
    // [reference: https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html]
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

}

fn calculate_length(s: &String) -> usize {
    s.len()
}