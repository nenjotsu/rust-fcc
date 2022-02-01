use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

/**
Implementing Deref Trait
*/
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rsut"));
    hello(&m);
    drop_trait();
}

// Rust does deref coercion when it finds types and trait implementations in three cases:
// - From &T to &U when T: Deref<Target=U>
// - From & mut T to &mut U when T:
//   DerefMut<Target=U>
// The first two cases are the same except for mutability.
// The first case states that if you have
fn hello(name: &str) {
    println!("hello, {}", name);
}

// drop trait
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("dropping with data {}", self.data);
    }
}

#[allow(unused_variables)]
fn drop_trait() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    drop(c);
    println!("created");
}
