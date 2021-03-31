#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    println!("Hello, world!");
    let rect1 = Rectangle {
        width: 100,
        height: 200,
    };
    println!("{}", area(&rect1));
    println!("{:#?}", rect1);
}

// Reference argument to not
// move ownership.
fn area(rect: &Rectangle) -> u32 {
    rect.height * rect.width
}
