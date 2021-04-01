fn main() {
    let v1 = Vector2d { x: 2, y: 3 };
    let v2 = Vector2d { x: 10, y: 20 };
    let v3 = sum(&v1, &v2);
    println!("After addition 1: {} {}", v3.x, v3.y);
    // let v4 = v1;
    // let v5 = v2;
    let v3 = v1 + v2;
    println!("After addition 2: {} {}", v3.x, v3.y)
}

// fn adder<T>(arg1: T, arg2: T) -> T {
//     return arg1 + arg2;
// }

struct Vector2d {
    x: i32,
    y: i32,
}

impl std::ops::Add for Vector2d {
    type Output = Vector2d;

    fn add(self, rhs: Vector2d) -> Vector2d {
        Vector2d {
            x: rhs.x + self.x,
            y: rhs.y + self.y,
        }
    }
}

impl std::clone::Clone for Vector2d {
    fn clone(&self) -> Vector2d {
        Vector2d {
            x: self.x,
            y: self.y,
        }
    }
}

// This works but seems dumb, do we need to clone?
fn sum<T: std::ops::Add + std::clone::Clone>(x: &T, y: &T) -> <T as std::ops::Add>::Output {
    let xtemp = x.clone();
    let ytemp = y.clone();
    xtemp + ytemp
}

