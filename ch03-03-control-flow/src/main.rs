fn main() {
    let number: i32 = 3;

    // Rust不会自动尝试将非布尔类型转换为布尔类型。
    // 您必须明确并始终提供 if 布尔值作为其条件。

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    loop_function();

    while_function();

    for_function()

}

fn loop_function() {

    let mut counter = 0;

    let result = loop {

        counter += 1;

        if counter == 10 {
            break counter * 2;
        }

    };

    println!("result = {}", result);

}

fn while_function() {

    let mut number = 3;

    while number != 0 {
        println!("{}", number);
        number -= 1;
    }

    println!("left");

}

fn for_function() {

    let array = [10, 20, 30, 40, 50];

    for element in array {
        println!("value: {}", element);
    }

    println!("~~~~~~~~~~~~~~~~~~~~~~~~~");

    // 反转数组
    for i in (0..array.len()).rev() {
        println!("value: {}", array[i]);
    }

}
