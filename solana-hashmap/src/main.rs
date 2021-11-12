use borsh::{BorshDeserialize, BorshSerialize};
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
/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GreetingAccount {
    /// number of greetings
    pub counter: u32,
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

    let mut k = vec![String::from("test")];
    println!("{}", k.len());
    k.push(String::from("another"));
    println!("{}", k.len());

    // greeting_account.counter += 1;
    //let mut greeting_account = GreetingAccount::try_from_slice();
    let mut greeting_account = GreetingAccount { counter: 2 };
    println!("Original again {}", greeting_account.counter);

    greeting_account.counter += 1;
    let mut result = Vec::with_capacity(1000);

    greeting_account.serialize(&mut result);

    let deserialized = GreetingAccount::try_from_slice(&result).unwrap();
    println!("Deserialized again {}", deserialized.counter);
}
// let unwrapped = match deserialized {
//     Ok(t) => t,
//     Err(e) => {
//         println!("Error")
//     }
// };
// unwrapped
