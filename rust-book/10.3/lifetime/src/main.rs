fn main() {
    //bad();
    let str1 = "test";
    let str2 = "test2";
    let resp = longest(str1, str2);
    println!("Longest: {}", resp);
}

// fn bad() {
//     let r;
//     {
//         let x = 5;
//         r = &x;
//     }
//     println!("{}", r);
// }

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
