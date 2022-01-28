use std::fmt::{Display, Debug};

#[allow(dead_code)]
#[allow(unused_variables)]
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
  // ..
  0
}
#[allow(dead_code)]
#[allow(unused_variables)]
fn some_function_refactor<T, U>(t: &T, u: &U) -> i32
  where T: Display + Clone,
        U: Clone + Debug
{
  //...
  0
}