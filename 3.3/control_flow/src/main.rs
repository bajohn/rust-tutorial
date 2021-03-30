fn main() {
    let cond = true;

    // let res = if cond { 3 } else { "nope" }; // Throws error, cool!
    let res = if cond { 3 } else { 4 };

    println!("Result {}", res);

    let mut i = 0;
    let res = loop {
        i += 1;
        if i > 5 {
            break i;
        }
    };

    println!("Loop Result {}", res);

    for number in 1..4 {
        println!("Counter {}", number)
    }
}
