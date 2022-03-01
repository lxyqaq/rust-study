const NUMBER: i32 = 6;

fn main() {

    let mut x = 7;

    println!("x = {}", x);

    x = NUMBER;

    println!("x = {}", x);

    // 可以通过使用相同变量的名称并重复使用 let 关键字来隐藏变量

    let spaces = "       ";

    let spaces = spaces.len();

    println!("spaces = {}", spaces);

}
