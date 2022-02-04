#![allow(dead_code)]
fn main() {
    println!("dynamically sized types");

    // = help: the trait `Sized` is not implemented for `str`
    // = note: all local variables must have a statically known size
    // = help: unsized locals are gated as an unstable feature
    let _s1: &str = "Hello there!"; // str this will error:
    let _s2: &str = "How's it going?"; // to solve this, use borrow str &str
}

// ?Sized = T is maybe sized or not
fn generic<T: ?Sized>(_t: &T) {
    // --snip--
}
