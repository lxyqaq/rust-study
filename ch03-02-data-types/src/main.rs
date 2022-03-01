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

    // Tuple type
    let tup: (i32, f64, u8) = (500, 2.3, 1);

    println!("{}", tup.0);
    println!("{}", tup.1);
    println!("{}", tup.2);

    //Array type
    let array: [i32; 5] = [1, 3, 9, 11, 193];
    println!("{}", array[0]);

}
