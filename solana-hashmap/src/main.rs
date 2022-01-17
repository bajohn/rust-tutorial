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
    pub aword: String,
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
    let another = vec![4, 5, 6, 7, 8];
    println!("{}", another[1..4][1]);
    k.push(String::from("another"));
    println!("{}", k.len());

    // greeting_account.counter += 1;
    //let mut greeting_account = GreetingAccount::try_from_slice();
    let mut greeting_account = GreetingAccount {
        aword: String::from("ohi"),
    };
    println!("Original  {}", greeting_account.aword);

    greeting_account.aword = String::from("another one");
    let mut result = Vec::with_capacity(1000);

    greeting_account.serialize(&mut result);
    println!("Origin length {}", result.len());

    let resp = truncate_vec(&result[..]);
    let truncatedVec = Vec::from(&result[0..resp]);
    println!("Final {}", truncatedVec.len());
    let deserialized = GreetingAccount::try_from_slice(&truncatedVec).unwrap();
    println!("Deserialized again {}", deserialized.aword);
}

fn truncate_vec(vecIn: &[u8]) -> usize {
    let mut i = vecIn.len();
    let mut truncIdx = 0;
    let mut ret: usize = 0;
    while i > 0 {
        i -= 1;
        match vecIn.get(i) {
            Some(num) => {
                if *num != 0 {
                    ret = i+1;
                    break;
                }
                println!("Found {}", num);
            }
            None => println!("Not found!"),
        }
    }
    ret
}
