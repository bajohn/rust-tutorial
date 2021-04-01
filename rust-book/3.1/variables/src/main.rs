fn main() {
    let x = 5;
    let x = x + 1;
    println!("The value of x is {}", x);
    let x = 10;
    println!("The value of x is {}", x);

    let y: f32 = 3.0;
    let y = y * 2.0;
    println!("The value of y is {}", y);

    let c = '🎉';
    println!("party {}", c);

    let tup = (3, 'a');
    println!("tup {}", tup.1);

    let arr: [i32; 3] = [5, 6, 7];
    let mut arr2 = [100;5];
    arr2[4] = 140;
    println!("arr {} {}", arr[2], arr2[4]);
}
