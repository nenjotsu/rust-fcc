#![allow(dead_code)]
mod gen;

// using generic type, can use different type inference
pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}

struct Counter {}

impl Iterator<u32> for Counter {
    fn next(&mut self) -> Option<u32> {
        Some(0)
    }
}

impl Iterator<u16> for Counter {
    fn next(&mut self) -> Option<u16> {
        Some(0)
    }
}

fn main() {
    println!("Hello, world!");
}
