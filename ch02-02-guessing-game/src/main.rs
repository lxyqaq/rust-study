use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {

    println!("猜数游戏开始！");

    let secret_number = rand::thread_rng().gen_range(1..100);

    loop {
        println!("猜测一个数字: ");

        let mut number = String::new();

        io::stdin().read_line(&mut number).expect("无法读取");

        let number: u32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("你读取的数字是: {}", number);

        match number.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }

}
