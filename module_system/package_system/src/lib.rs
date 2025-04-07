mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting; // Corrected line

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist(); // Now we can call the function
}

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order()
    }

    fn cook_order() {}
}

mod house_back {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        fn summer(toast: &str) -> Breakfast {
            Breakfast {
                taost: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_restaurant() {
    let mut meal = house_back::Breakfast::summer("Rype");
    meal.toast = String::from("Wheat"); // if toast is public then only we can modify
}
