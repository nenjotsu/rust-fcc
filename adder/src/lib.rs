//! # adder
//!
//! `adder` a library for example.

#[derive(Debug)]
#[allow(dead_code)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[allow(dead_code)]
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[doc = "Add two to the number given."]
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = adder::add_two(arg);
///
/// assert_eq!(7, answer);
/// ```
pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[allow(unused_variables)]
pub fn greeting_fail(name: &str) -> String {
    format!("Hello!")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2))
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }

    #[test]
    #[should_panic]
    // #[should_panic("does not contain name, the value was {}")]
    // #[ignore]
    fn greeting_fails() {
        let result = greeting_fail("Carol");
        assert!(
            result.contains("Carol"),
            "does not contain name, the value was {}",
            result
        );
    }
}
