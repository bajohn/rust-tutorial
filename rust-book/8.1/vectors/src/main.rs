fn main() {
    let mut v = vec![4, 5, 6];

    let k = v.get(1);
    let mut err = v[1]; // will error if out of bounds
    err = 100;

    let err2 = v[1]; // will error if out of bounds
    match k {
        Some(num) => println!("{}", num),
        None => println!("Not found!"),
    }

    println!("Sketch {} {}", v[1], err);

    for i in &mut v {
        *i = *i + 1;
    }

    println!("Post loop {}", v[1]);

    #[derive(Debug)]
    enum Cell {
        Text(String),
        Numeric(i32),
    }

    let cells = vec![Cell::Text(String::from("Test")), Cell::Numeric(4)];
    // let badcells = vec![1, "cat"]; // not allowed! One type per vector.
    println!("text {:#?}", cells[0]);
}
