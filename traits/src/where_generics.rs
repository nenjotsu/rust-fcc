fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
  // ..
}

fn some_function_refactor<T, U>(t: &T, u: &U) -> i32
  where T: Display + Clone,
        U: Clone + Debug
{
  //...
}