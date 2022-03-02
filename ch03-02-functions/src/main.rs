fn main() {

    next_function(13, 1.3);

    println!("return_function = {}", return_function(3));

}

fn next_function(x: i64, y: f64) {
    println!("{}", ("Hello Function!"));
    println!("x = {}, y = {}", x, y);
}

fn return_function(x: i32) -> i32 {
    x + 1
}
