struct User {
    email: String,
    username: String,
    active: bool,
}

fn main() {
    println!("Hello, world!");
    let user1: User = User {
        email: String::from("test@me.com"),
        username: String::from("test"),
        active: true,
    };
    println!("{} {} {}", user1.username, user1.email, user1.active);
}
