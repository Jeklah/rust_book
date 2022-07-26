mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}



// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//  
//         fn seat_at_table() {}
//     }
//  
//     mod serving {
//        fn take_order() {}
// 
//        fn serve_order() {}
// 
//        fn take_payment() {}
//     }
// }

fn deliver_order() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
         pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }


    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}


// pub fn eat_at_restaurant() {
//     // Absolute path
//     crate::front_of_house::hosting::add_to_waitlist();
// 
//     // Relative path 
//     front_of_house::hosting::add_to_waitlist();
//     
//     // Can do this due to the use crate above.
//     hosting::add_to_waitlist();
// 
//     // Order a breakfast in the summer with Rye toast 
//     let mut meal = back_of_house::Breakfast::summer("Rye");
//     // Change our mind about wheet bread we'd like 
//     meal.toast = String::from("Wheat");
//     println!("I'd like {} toast please", meal.toast);
// 
//     let order1 = back_of_house::Appetizer::Soup;
//     let order2 = back_of_house::Appetizer::Salad;
//     
//     // the next line won't compile if we uncomment it; we're not allowed
//     // to see or modify the seasonal_fruit that comes with the meal 
//     // meal.seasonal_fruit = String::from("blueberries");
// }


// mod customer {
//     pub fn eat_at_restaurant() {
//         hosting::add_to_waitlist();  // will not compile because the use statement is not at the same level as customer
//     }
// }
