#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Quarter(UsState),
}

fn value(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Quarter(state) => {
            println!("{:#?}", state);
            25
        }
    }
}

fn negate(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(-i),
        None => None,
    }
}

fn main() {
    let al_quarter = Coin::Quarter(UsState::Alabama);
    let val = value(&al_quarter);
    println!("{}", val);

    let some = negate(Some(3));
    println!("{}", some.unwrap());
    // let none = negate(None);
    // println!("{}", none.unwrap()); throws error!
    if 10 == 10 {
        println!("yup")
    }
    if let Coin::Quarter(UsState::Alabama) = al_quarter {
        println!("yup yup yup")
    }
}
