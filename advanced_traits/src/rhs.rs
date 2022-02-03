// default generic type parameter
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point; // default generic type
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.y,
            y: self.y + other.x,
        }
    }
}

// Rhs = Right Hand Side
trait Add<Rhs = Self> {
    type Output;
    fn add(self, rhs: Rhs) -> Self::Output;
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}
