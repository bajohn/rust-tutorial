use std::error::Error;
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    //panic!("BURN");
    let f = File::open("tet.txt");
    match f {
        Ok(res) => println!("Ok"),
        Err(err) => match err.kind() {
            //  println!("Not found! {}", err.description())
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => println!("file created!"),
                Err(e) => println!("err creating file"),
            },
            _ => println!("Unknown error!"),
        },
    }
}
