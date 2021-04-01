fn main() {
    let immut = "Hello, world!"; // oh this is an immutable reference to the binary!
    println!("{} ", immut);
    let mut s = String::from("Test string");

    let r = &mut s;
    r.push_str("SSSSSSSSS");
    let sliced = &mut r[2..18];
    println!("{} ", sliced);
}
