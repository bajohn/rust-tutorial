struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn side_longer(&self, comparer: u32) -> bool {
        comparer > self.width && comparer > self.height
    }

    fn square(side: u32) -> Rectangle {
        Rectangle {
            width: side,
            height: side,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 10,
        height: 20,
    };
    let area = rect1.area();

    println!("{}", area);

    let square1 = Rectangle::square(3);

    println!("{}", square1.area());

    println!("{}", rect1.side_longer(1000));
}
