use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number");

    let secret = rand::thread_rng().gen_range(1, 101);
    println!("Secret {}", secret);

    loop {
        println!("Input a guess");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Error!");
        println!("Guess {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number");
                continue;
            }
        };
        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                break;
            }
            Ordering::Greater => println!("Too large!"),
        };
    }
    println!("Win!");
}
