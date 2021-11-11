use rand::Rng;
use std::str;

fn main() {
    println!("{}", get_salt());
}

fn get_salt() -> String {
    let mut rng = rand::thread_rng();
    let mut ret = String::from("");
    for _ in 1..4 {
        let i: u8 = rng.gen_range(97..123);
        let utf8_vec = vec![i];
        let s = str::from_utf8(&utf8_vec).unwrap();
        ret.push_str(s);
    }
    ret
}
