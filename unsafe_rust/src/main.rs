#![allow(dead_code)]
#![allow(unused_variables)]

fn main() {
    // unsafe keyword gives you five abilities
    println!("derefence a raw pointer");
    println!("call an unsafe function or method");
    println!("access or modify a mutable static variable");
    println!("implement unsafe trait");
    println!("access fields of union");
}

fn derefence_raw_pointer() {
    let mut num = 5;
    // *const is a raw pointers
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // let address = 0x012345usize;
    // let r3 = address as *const i32;

    unsafe {
        println!("r1 is {}", *r1);
        println!("r2 is {}", *r2);
    }
}

fn calling_unsafe_method() {
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }

    safe_split();
}

// example of safe vs unsafe method
fn safe_split() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}


// fn split_at_mut_safe(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//     let len = slice.len();
//     assert!(mid <= len);
    
//     // error: cannot borrow, refactor is split_at_mut_unsafe
//     // (&mut slice[..mid], &mut slice[mid..]) 
// }

use std::slice;

fn split_at_mut_unsafe(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let nn = 0;
    let ptr = slice.as_mut_ptr();
    assert!(mid <= len);
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}

// use ffi = foreign function interface
fn from_external_language() {
    unsafe {
        println!("absolute value of -3 according to C: {}", abs(-3));
    }
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("just called a Rust funcion from C");
}

// access or modify a mutable static variable
static HELLO_WORLD: &str = "Hello, world";

static mut COUNTER: u32 = 0;
fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

// static variables = global variable,
// similar to const but have fixed address in memory, vs const have duplicate data
// static if mutable is unsafe, vs const is safe
fn modify_static_var() {
    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

// implement unsafe trait
unsafe trait Foo {
    // TODO: ...
}

unsafe impl Foo for i32 {
    // TODO: ...
}

// access fields of union
