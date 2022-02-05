// #![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

pub struct StrSplit {
    pub fn new(haystack: &str, delimeter: &str) -> Self {
        
    }
}

impl Iterator for StrSplit {
    type Item<'a> = &'a str;
    fn next(&mut self) -> Option<'a Self::Item> {
        
    }
}

#[test]
fn test_it_works() {
    let haystack = "a b c d e";
    let letters in StrSplit::new(haystack, " ");
    assert_eq!(letters, vec!["a", "b", "c", "d", "e"].into_iter());
}