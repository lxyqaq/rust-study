fn main() {

    // move occurs because `number1` has type `String`,
    // which does not implement the `Copy` trait

    // let number1 = String::from("Hello World!");
    // let number2 = number1;
    // println!("{}", number1);

    let number1 = 42;
    let _number2 = number1;
    println!("{}", number1);

}
