pub mod hosting;

pub mod serving {
    fn take_order() {}

    fn serve_order() {}

    fn take_payment() {}

    pub mod back_of_house {
        pub struct Breakfast {
            pub toast: String,
            seasonal_fruit: String,
        }

        pub enum Appetizer {
            Soup,
            Salad,
        }

        impl Breakfast {
            pub fn summer(toast: String) -> Breakfast {
                Breakfast {
                    toast,
                    seasonal_fruit: "peaches".to_string(),
                }
            }
        }

        fn fix_incorrect_order() {
            cook_order();
            super::serve_order();
        }

        fn cook_order() {}
    }
}

