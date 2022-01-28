use std::fmt::Display;

#[allow(dead_code)]
struct Pair<T> {
  x: T,
  y: T
}

#[allow(dead_code)]
impl<T> Pair<T> {
  fn new(x: T, y: T) -> Self {
    Self { x, y }
  }
}

#[allow(dead_code)]
impl<T: Display + PartialOrd> Pair<T> {
  fn amp_display(&self) {
      if self.x >= self.y {
        println!("the largest member is x = {}", self.x);
      } else {
        println!("the largest member is y = {}", self.y);

      }
  }
}