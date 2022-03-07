fn main() {
    println!("{}", ("Hello World!"));

    let mut user1 = User {
        username: String::from("Nathan Liu"),
        active: true,
        email: String::from("xiangyu.liu.tus@hotmail.com"),
        sign_in_count: 293,
    };

    user1.email = String::from("xiangyu.liu@gmail.com");

    // Creating Instances From Other Instances With Struct Update Syntax
    let user2 = User {
        email: String::from("example@gmail.com"),
        ..user1
    };

    println!("{:?}", user2);

}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 300,
    }
}

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}