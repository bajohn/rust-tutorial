fn main() {
    let x = 5;

    let y = x + 1;
    println!("x, y: {}, {}", x, y);

    let s1 = String::from("Hi!");
    let s2 = s1.clone();
    let s3 = s2; // this drops s2!
    println!("{} {}", s1, s3);

    let s4 = String::from("testing");
    let s5 = rescope(s4);
    println!("{}", s5);

    let last = bigscope(s5);
    println!("{} {}", last.0, last.1);
}

fn rescope(mut x: String) -> String {
    x.push_str(" and more");
    x
}

fn bigscope(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
