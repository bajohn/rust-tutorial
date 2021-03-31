#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }

    pub fn eat_at_restaurant() {
        crate::front_of_house::hosting::add_to_waitlist(); //absolute
        hosting::add_to_waitlist(); //relative
        super::front_of_house::hosting::add_to_waitlist(); // dumb way to do it
    }
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: String) -> Breakfast {
            Breakfast {
                toast,
                fruit: String::from("peaches"),
            }
        }
    }
}

pub fn order() {
    let mut breakfast = back_of_house::Breakfast::summer(String::from("rye"));

    println!("{}", breakfast.toast);
    breakfast.toast = String::from("test");
}

use self::front_of_house::hosting as fh_hosting;

pub fn use_order() {
    fh_hosting::add_to_waitlist();
}

mod remote;
use crate::remote::phone;

pub fn handle_order() {
    phone::phone_order();
}
