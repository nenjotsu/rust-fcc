# rust-fcc
rust-fcc

source [https://www.freecodecamp.org/news/rust-in-replit/](https://www.freecodecamp.org/news/rust-in-replit/)

# Basic of Rust
## Variables
```rust
let my_variable = 0;
const MY_CONSTANT: u8 = 0;
static MY_STATIC: u8 = 0;

let mut my_mutable_variable = 0;
```



| Object 	| Casing |
|---|---|
|Variables 	|snake_case|
|Functions| 	snake_case|
|Files| 	snake_case|
|Constants| 	SCREAMING_SNAKE_CASE|
|Statics| 	SCREAMING_SNAKE_CASE|
|Types| 	PascalCase|
|Traits| 	PascalCase|
|Enums| 	PascalCase|

## Functions
```rust
fn main() {
  // your code
}
```
```rust
fn main() -> () { // Unnecessary return type
  my_func();
}

fn my_func() -> u8 { // using `return` keyword
  return 0;
}
```
```rust
// return an expression, semi-colon should be excluded
fn my_fnuc() -> u8 {
  0
}
```
```rust
fn main() {
  let _unused_variable = my_func(10); // underscore before a variable name is a convention to indicate that the variable is unused
}

fn my_func(x: u8) -> i32 { // Function parameters are typed using the : syntax
  x as i32 // `as` keyword asserts the type of the expression, provided the type conversion is valid
}
```
## Strings and Slices
```rust
let my_str: &str = "Hello, world!"; //  is a reference to a `string literal`

let my_string: String = String::from("Hello, world!"); // is an instance of the `String` struct
```
An important distinction between the two is that `my_str` is stack stored, and `my_string` is heap allocated. This means `my_str`'s value cannot change, and its size is fixed, whilst `my_string` can have an unknown size at compile time.

The string literal is also known as a *string slice*. This is because a `&str` refers to part of a string. Generally, this is how arrays and strings are similar:

```rust
let my_string = String::from("The quick brown fox");
let my_str: &str = &my_string[4..9]; // "quick"

let my_arr: [usize; 5] = [1, 2, 3, 4, 5];
let my_arr_slice: &[usize] = &my_arr[0..3]; // [1, 2, 3]
```
The `[T; n]` notation is used to create an array of `n` elements of type `T`.

## `char` Type
A `char` is a USV (Unicode Scalar Value), which is represented in unicode with values like `U+221E` – the unicode for '∞'. You can think of a collection or array of `char`s as a string:
```rust
let my_str: &str = "Hello, world!";

let collection_of_chars: &str = my_str.chars().as_str();
```
## Number Types
- Unsigned Integers: `u8`, `u16`, `u32`, `u64`, `u128`
- Signed Integers: `i8`, `i16`, `i32`, `i64`, `i128`
- Floating Point Numbers: `f32`, `f64`

Unsigned integers only represent positive whole numbers.

Signed integers represent both positive and negative whole numbers.

And floats only represent positive and negative fractions.

## Structs
A struct is a custom data type used to group related data. You have already come across a struct in the Strings and Slices section:
```rust
struct String {
  vec: Vec<u8>,
}
```
The `String` struct consists of a `vec` field, which is a `Vec` of `u8`s. The `Vec` is a dynamically-sized array.

An instance of a struct is then declared by giving values to the fields:
```rust
struct MyStruct {
  field_1: u8,
}

let my_struct = MyStruct { field_1: 0, };
```

Previously, the `String` struct was used with its `from` function to create a `String` from a `&str`. This is possible, because the `from` function is implemented for `String`:

```rust
impl String {
  fn from(s: &str) -> Self {
    String {
      vec: Vec::from(s.as_bytes()),
    }
  }
}
```
You use the `Self` keyword in place of the type of the struct.

Structs can also take other variants:
```rust
struct MyUnitStruct;
struct MyTupleStruct(u8, u8);
```

## Enums
Similar to other languages, enums are useful for acting as types and as values.
```rust
enum MyErrors {
  BrainTooTired,
  TimeOfDay(String)
  CoffeeCupEmpty,
}

fn work() -> Result<(), MyErrors> { // Result is also an enum
  if state == "missing semi-colon" {
    Err(MyErrors::BrainTooTired)
  } else if state == "06:00" {
    Err(MyErrors::TImeOfDay("It's too early to work".to_string()))
  } else if state == "22:00" {
    Err(MyErrors::TimeOfDay("It's too late to work".to_string()))
  } else if state == "empty" {
    Err(MyErrors::CoffeeCupEmpty)
  } else {
    Ok(())
  }
}
```
## Macros
A macro is similar to a function, but you can think of it as a piece of code which writes other code. 
- Macros are called using a bang (`!`)
- Macros can take a variable number of arguments, while functions in Rust cannot
```rust
let my_str = "Hello, world!";
println!("{}", my_str);
format!("{my_str}")
```
```rust
let am_i_an_error = true;

if (am_i_an_error) {
  panic!("There was an error");
}
```
## Ownership
There are three main ownership rules:
- Each value in Rust has a variable that's called its owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

This is how Rust gets away with not having a typical garbage collector, whilst also not requiring the programmer to explicitly manage memory. Here is an example of ownership:
```rust
fn main() { // first_string is not declared yet -> has no value
  let first_string = String::from("freeCodeCamp"); // first_string is now owner of the value "freeCodeCamp"
  let second_string = first_string; // second_string takes ownership of the value "freeCodeCamp"

  println!("Hello, {}!", first_string); // first_string is NOT valid, because the value was moved to second_string
}
```

As the `println!` macro tries to refer to an invalid variable, this code does not compile. To fix this, instead of moving the value of `first_string` into `second_string`, `second_string` can be assigned a reference to `first_string`:
```rust
fn main() {
  let first_string: String = String::from("freeCodeCamp");
  let second_string: &String = &first_string; // first_string is still the owner of the value "freeCodeCamp"

  println!("Hello, {}!", first_string);
}
```
The ampersand (`&`) indicates that the value is a reference. That is, `second_string` no longer takes ownership of `"freeCodeCamp"`, but, instead, points to the same point in memory as `first_string`.