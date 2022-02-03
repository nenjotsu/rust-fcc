#![allow(unused_variables)]
#![allow(dead_code)]

// newtype pattern
use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

struct Age(u32);
struct ID(u32);

// never type in panic macro
// impl<T> Option<T> {
//     pub fn unwrap(self) -> T {
//         match self {
//             Some(val) => val,
//             None => panic!("called `Option::unwrap()` on a `None` value"),
//         }
//     }
// }

// never type in loop
// loop {
//     println!("and ever");
// }

pub fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);

    // adition in new type pattern
    type Kilometers = i32; // new type synonym to i32
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);

    // benefits of new type: to reduce duplication
    type Thunk = Box<dyn Fn() + Send + 'static>; // type alias
    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_long_type(f: Thunk) {
        // TODO: ...
    }

    fn returns_long_type() -> Thunk {
        // TODO: ...
        unimplemented!() // TODO
    }

    // never type
    fn never_return() -> ! {
        unimplemented!() // TODO
    }

    // never type: example
    // let game_in_progress = false;
    // while game_in_progress {
    //     let guess: u32 = match guess.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => continue, // has never type
    //     };
    // }
}
