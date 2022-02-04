#![allow(dead_code)]

mod closures;

fn add_one(x: i32) -> i32 {
    x + 1
}

// function pointer
// fn is a type
// function pointers implement all 3 closure traits
// fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
//     f(arg) + f(arg)
// }

fn do_twice<T>(f: T, arg: i32) -> i32
where
    T: Fn(i32) -> i32,
{
    f(arg) + f(arg)
}

// there are three closure traits
// Fn, FnMut, and FnOnce

fn func_pointer() {
    let answer = do_twice(add_one, 5);
    println!("the answer is = {}", answer);
}

fn main() {
    closures::main();
    closures::func_pointer_tuple();
    // closures::returns_closures(11);
}
