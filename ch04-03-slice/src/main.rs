fn main() {
    let str = String::from("Hello World");

    // &str[0..5] = &str[..5]
    let slice1 = &str[0..5];
    println!("slice1 = {}", slice1);

    // Hello
    let slice2 = &str[..5];
    println!("slice2 = {}", slice2);

    // World
    // &str[6..str.len()]
    let slice3 = &str[6..];
    println!("slice3 = {}", slice3);

    // Hello World
    let slice4 = &str[..];
    print!("slice4 = {}", slice4);

}
