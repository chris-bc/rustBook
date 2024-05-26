mod front_of_house;

mod back_of_house {
    pub enum Appetiser {
        Soup,
        Salad,
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
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

fn deliver_order() {}

mod customer {
    use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        crate::front_of_house::hosting::add_to_waitlist();

        super::front_of_house::hosting::add_to_waitlist();

        hosting::add_to_waitlist();

        // Order a breakfast in Summer with Rye toast
        let mut meal = super::back_of_house::Breakfast::summer("Rye");
        // Change our mind about bread
        meal.toast = String::from("Wheat");
        println!("I'd like {} toast please", meal.toast);

        let order1 = super::back_of_house::Appetiser::Soup;
        let order2 = super::back_of_house::Appetiser::Salad;
    }
}