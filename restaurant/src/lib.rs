mod front_of_house;

use std::collections::HashMap;
use std::fmt;
use std::io::Result as IoResult;

// 重导出 re-exporting
pub use front_of_house::{hosting, serving};
use serving::back_of_house;

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    hosting::seat_at_table();

    // 相对路径
    self::front_of_house::hosting::seat_at_table();

    let mut meal = front_of_house::serving::back_of_house::Breakfast::summer("Rye".to_string());
    meal.toast = "Wheat".to_string();

    let a1 = front_of_house::serving::back_of_house::Appetizer::Soup;
    let a2 = front_of_house::serving::back_of_house::Appetizer::Salad;

    let mut map = HashMap::new();
    map.insert(1, "Rye".to_string());
}

fn fun1() -> fmt::Result {
    println!("fun1");
    Ok(())
}

fn fun2() -> IoResult<()> {
    println!("fun2");
    Ok(())
}