#[allow(dead_code)]
struct Point<T> {
  x: T,
  y: T
}
#[allow(dead_code)]
impl<U> Point<U> {
  fn x(&self) -> &U {
      &self.x
  }
}

#[allow(dead_code)]
fn main() {
  let p1 = Point { x:5, y:10 };
  println!("{}", p1.x());
  let p2 = Point { x:5.0, y:10.0 };
  println!("{}", p2.y());
  // let p3 = Point { x:5, y:10.0 };

}

// concrete type parameter
#[allow(dead_code)]
impl Point<f64> {
    fn y(&self) -> f64 {
        self.y
    }
}