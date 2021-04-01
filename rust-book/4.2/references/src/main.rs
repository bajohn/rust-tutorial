fn main() {

    let mut test = String::from("test");
    //let ref_str = &test; // can't have mutable reference followed by mutation!
    test.push_str("test2");
    let ref_str = &mut test; // the original "test" is no longer the owner.
    ref_str.push_str("more");

    println!("{}", ref_str); // could not print "test" here.
}

// This function doesn't compile
// because the return references
// a value that goes out of scope!
// fn dangle()-> &String {
//     let s = String::from("testttt");
//     &s
// }

// fn reffunc(s: &mut String) {
//     println!("{}", s);
// }
