use std::fmt::Display;

struct Pair<T> {
  x: T,
  y: T
}

impl<T> Pair<T> {
  fn new(x: T, y: T) -> Self {
    Self { x, y }
  }
}

impl<T: Display + PatialOrd> Pair<T> {
  fn amp_display(&self) {
      if self.x >= self.y {
        println!("the largest member is x = {}", self.x);
      } else {
        println!("the largest member is y = {}", self.y);

      }
  }
}