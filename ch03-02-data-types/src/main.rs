fn main() {

    // consider giving `guess` a type
    // let guess = "42".parse().expect("Not a number");
    let guess: u32 = "42".parse().expect("Not a number");

    println!("guess = {}", guess);

    let x = 2.0;

    println!("{}", x);

    let y: f32 = 3.0;

    println!("{}", y);

    let t = true;

    println!("{}", t);

    let f: bool = false;

    println!("{}", f);

    let c = 'z';

    println!("{}", c);

    let cry = '😂';

    println!("{}", cry);

}
