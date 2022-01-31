fn main() {
    let b = Box::new(5);
    println!("b = {b}");

    box_list();
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn box_list() {
    let bl = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("bl = {:?}", bl);
}
