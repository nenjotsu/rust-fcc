// #![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

// Note:
// str === -> [char]
// &str === ->  &[char] <--- stack allocated
// String === -> Vec<char>  <--- heap allocated, dynamically expendable and contractable, shrink and grow
//
// String -> &str (cheap -- AsRef)
// &str -> String (expensive -- memcpy)
mod lifetime_and_generics;

#[derive(Debug)]
pub struct StrSplit<'h, 'd> {
    remainder: Option<&'h str>,
    delimiter: &'d str,
}

impl<'h, 'd> StrSplit<'h, 'd> {
    pub fn new(haystack: &'h str, delimiter: &'d str) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

// '_ ignored
impl<'h> Iterator for StrSplit<'h, '_>
// where
//     'd: 'h, // means, d is longer than h, or d implements h
{
    type Item = &'h str;
    // fn next_old(&mut self) -> Option<Self::Item> {
    //     // this is the same as
    //     // if let Some(remainder) = &mut self.remainder {
    //     if let Some(ref mut remainder) = self.remainder {
    //         if let Some(next_delim) = remainder.find(self.delimiter) {
    //             let until_delimiter = &remainder[..next_delim];

    //             // &mut &'a str = &'a str
    //             *remainder = &remainder[(next_delim + self.delimiter.len())..];
    //             Some(until_delimiter)
    //         } else {
    //             // take is, returning the Option<T>,
    //             // if it's None it will return None
    //             // or if it's Some it will replace the None what's in there,
    //             // here it's empty string ""
    //             // impl<T> Option<T> { fn take(&mut self) -> Option<T> }
    //             self.remainder.take()
    //         }
    //     } else {
    //         None
    //     }
    // }

    // refactor
    fn next(&mut self) -> Option<Self::Item> {
        // let ref remainder = self.remainder.as_mut()?;
        // same as
        let remainder = self.remainder.as_mut()?;

        if let Some(next_delim) = remainder.find(self.delimiter) {
            let until_delimeter = &remainder[..next_delim];
            *remainder = &remainder[(next_delim + self.delimiter.len())..];
            Some(until_delimeter)
            // Some(self.delimiter)
        } else {
            self.remainder.take()
        }
    }
}

#[allow(unused)]
fn until_char(s: &str, c: char) -> &str {
    let delim = format!("{}", c);
    StrSplit::new(s, &delim)
        .next()
        .expect("StrSplit always gives at least one result")
}

#[test]
fn test_until_chart() {
    assert_eq!(until_char("hello world", 'o'), "hell");
}

#[test]
fn test_it_works() {
    let haystack = "a b c d e";
    let letters = StrSplit::new(haystack, " ");
    assert!(letters.eq(vec!["a", "b", "c", "d", "e"].into_iter()));

    // or
    let letters1: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters1, vec!["a", "b", "c", "d", "e"]);
}

#[test]
fn test_it_works_refactor() {
    let haystack = "a b c d e";
    // or
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
}

#[test]
fn test_it_works_tail() {
    let haystack = "a b c d ";
    // or
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", ""]);
}
