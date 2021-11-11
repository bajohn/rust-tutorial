
fn main() {
    let weird_bool = do_iter();
    let ret = match weird_bool {
        true => 10,
        false => 20,
    };
    println!("The ret is {}", ret);
}

// fn get_bool() -> bool {
//     let ret = true;
//     return ret;
// }

fn do_iter() -> bool {
    let comparer = 7;
    let arr = vec![4, 6, 8];
    let mut iter = arr.iter();
    let mut cur = iter.next();
    let mut joined = false;

    while !cur.is_none() {
        if cur == Some(&comparer) {
            joined = true;
        }
        cur = iter.next();
    }
    joined
}
