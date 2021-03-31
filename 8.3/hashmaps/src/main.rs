use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("red"), 10);
    scores.insert(String::from("blue"), 12);

    println!("{}", scores.get("red").unwrap()); // unsafe!
    scores.entry(String::from("red")).or_insert(100); // cool!
}
