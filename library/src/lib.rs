#[allow(dead_code)]
mod front_of_house {
  pub mod hosting {
    pub fn add_to_waitinglist() {}
    fn seat_at_table() {}
  }

  mod serving {
    fn take_order() {}
    fn serve_order() {}
    fn tale_payment() {}
  }
}

use self::front_of_house::hosting;
// use crate::front_of_house::hosting;

pub fn eat_at_restaurant_foh() {
  // absolute path
  crate::front_of_house::hosting::add_to_waitinglist();

  // relative path
  front_of_house::hosting::add_to_waitinglist();

  hosting::add_to_waitinglist();
  hosting::add_to_waitinglist();
  hosting::add_to_waitinglist();
}

fn serve_order() {}

#[allow(dead_code)]
mod back_of_house {

  fn fix_incorrect_order() {
    cook_order();
    super::serve_order(); // server accessing the parent
  }

  fn cook_order() {}

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
        seasonal_fruit: String::from("peaches")
      }
    }
  }
}

#[allow(unused_variables)]
pub fn eat_at_restaurant_boh() {
  let mut meal = back_of_house::Breakfast::summer("toast");
  meal.toast = String::from("wheat");

  let order1 = back_of_house::Appetizer::Soup;
  let order2 = back_of_house::Appetizer::Salad;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
