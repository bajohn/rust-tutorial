use std::collections::HashMap;

struct User {
    email: String,
    username: String,
    active: bool,
}

impl User {
    fn new(email: &str, username: &str, active: bool) -> User {
        User {
            email: String::from(email),
            username: String::from(username),
            active: active,
        }
    }
}

fn main() {
    println!("Hello, world!");

    let user1 = User::new("test", "test", true);
    let user2 = User::new("test2", "test3", true);

    let mut acc_store = HashMap::new();
    acc_store.insert(user1.email, true);
    acc_store.insert(user2.email, true);

    let key1 = acc_store.contains_key(&String::from("test"));
    let key2 = acc_store.contains_key(&String::from("wrong"));

    println!("{} ", key1);
    println!("{} ", key2);
}
