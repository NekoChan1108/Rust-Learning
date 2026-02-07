// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

//!
//!  lib crate example
//!
#[allow(unused)]
// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }
pub mod front_of_house;
use front_of_house::hosting;

#[allow(unused)]
fn serve_order() {}

mod back_of_house {

    #[derive(Debug)]
    pub struct Breakfast {
        pub toast: String,
        #[allow(unused)]
        seasonal_fruit: String,
    }

    #[derive(Debug)]
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

    #[allow(unused)]
    fn fix_incorrect_order() {
        cook_order();
        // 这两行的作用其实是一样的 因为 back_of_house 的夫模块就是 crate
        super::serve_order();
        crate::serve_order();
    }
    fn cook_order() {}
    mod xxx {
        #[allow(unused)]
        fn xxx() {
            // 类似于 ../../serve_order
            super::super::serve_order();
        }
    }
}

pub use back_of_house::Appetizer as A;
use back_of_house::Breakfast;
pub fn eat_at_restaurant() {
    // // 绝对路径
    // crate::front_of_house::hosting::add_to_waitlist();
    //
    // // 相对路径
    // front_of_house::hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    // 在夏天点一份黑麦面包作为早餐
    let mut meal = Breakfast::summer("Rye");
    // 更改我们想要的面包
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    dbg!(meal);

    // 如果取消下一行的注释，将会导致编译失败因为我们不被允许
    // 看到或更改随餐搭配的季节水果
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = A::Soup;
    let order2 = A::Salad;
    dbg!(order1, order2);
}
pub fn xxx() {
    let order1 = A::Soup;
    let order2 = A::Salad;
    dbg!(order1, order2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        eat_at_restaurant()
    }
}
