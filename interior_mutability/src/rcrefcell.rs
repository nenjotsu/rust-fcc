use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use std::cell::RefCell;
use List::{Cons, Nil};

pub fn refcell() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    // println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a)); // or a.clone()
                                                           // println!("count after creating b = {}", Rc::strong_count(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a)); // without Rc, value used here after move,
                                                           // println!("count after creating c = {}", Rc::strong_count(&a));

    *value.borrow_mut() += 10;
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    // println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
