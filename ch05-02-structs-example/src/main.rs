fn main() {

    let rect = Rectangle {
        width: 30,
        length: 20,
    };

    println!("area = {}", area(&rect));

    println!("{:#?}", rect);

}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.length
}
