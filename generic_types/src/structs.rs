#[allow(dead_code)]
struct Point<T, U> {
  x: T,
  y: U
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn main() {
  let p1: Point<i32, i32> = Point { x:5, y:10 };
  let p2 = Point { x:5.0, y:10.0 };
  let p3 = Point { x:5, y:10.0 };
}