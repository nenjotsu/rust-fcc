use std::fmt::Display;

fn main() {
  let r;
  {
    let x = 5;
    // r = &x; // err: `x` does not live long enough
    r = x;
  }
  println!("r: {}", r);
}

// fn lifetime_annotation() {
//   let string1 = String::from("abcd");

//   {
//     let string2 = String::from("xyz");
//     let result = longest(string1.as_str(), string2.as_str());
//     println!("the longest string is {result}");
//   }
// }

// dangling references
// fn lifetime_annotation_error() {
//   let string1 = String::from("abcd");
//   let result;
//   {
//     let string2 = String::from("xyz");
//     // result = longest(string1.as_str(), string2.as_str()); // err: `string2` does not live long enough
//     println!("the longest string is {result}");
//   }
//   println!("the longest string is {result}");
// }

// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//   if x.len() > y.len() {
//     x
//   } else {
//     y
//   }
// }

#[allow(dead_code)]
fn lifetime_annotation_fix() {
  let string1 = String::from("abcd");
  let result;
  {
    let string2 = String::from("xyz");
    result = longest_fix(string1.as_str(), string2.as_str()); // `string2` does not live long enough
  }
  println!("the longest string is {result}");
}

fn longest_fix<'a>(x: &'a str, _y: &str) -> &'a str {
  x
}


// lifetime annotation in struct definitions
#[allow(dead_code)]
struct ImportantExcerpt<'a> {
  part: &'a str,
}

#[allow(dead_code)]
impl<'a> ImportantExcerpt<'a> {
  fn return_part(&self, announcement: &str) -> &str {
    println!("attention please: {announcement}");
    self.part
  }
}

#[allow(unused_variables)]
#[allow(dead_code)]
fn struct_main() {
  let novel = String::from("Call me joe. Some years ago...");
  let first_sentence = novel.split('.').next().expect("Could not find");
  let i = ImportantExcerpt {
    part: first_sentence,
  };
  // you'll get an error here
}

// lifetime Ellision
// rules
// 1. Each parameter that is a reference gets its own lifetime parameter
// 2. If there is exactly one input lifetime parameter,
//    that lifetime is assigned to all output lifetime parameters;
// 3. If there are multiple input lifetime parameters, but one of them is
//    &self or &mut self the lifetime of self is assigned to all output
//    lifetime parameters.
#[allow(dead_code)]
fn first_word<'a>(s: &'a str) -> &'a str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[0..i];
    }
  }

  &s[..]
}

// static lifetime
#[allow(dead_code)]
fn longest_with_an_announcement<'a, T>(
  x: &'a str,
  y: &'a str,
  ann: T
) -> &'a str
where
    T: Display, // trait bounds, that implements Display
{
  println!("announcement! {ann}");
  if x.len() > y.len() {
    x
  } else {
    y
  }
}
