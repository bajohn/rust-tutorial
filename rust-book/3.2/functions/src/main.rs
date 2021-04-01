fn main() {
    func1({
        let temp = five() + 2;
        temp + 3
    });
}

fn func1(num: i32) {
    println!("Func 1 arg: {}", num);
}

fn five() -> i32 {
    5
}
