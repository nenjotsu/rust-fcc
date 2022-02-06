#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

#[derive(Debug)]
pub struct StrSplit<'h, D> {
    remainder: Option<&'h str>,
    delimiter: D,
}

impl<'h, 'd> StrSplit<'h, 'd> {
    pub fn new(haystack: &'h str, delimiter: &'d str) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

impl<'h> Iterator for StrSplit<'h, '_> {
    type Item = &'h str;

    // refactor
    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder.as_mut()?;

        if let Some(next_delim) = remainder.find(self.delimiter) {
            let until_delimeter = &remainder[..next_delim];
            *remainder = &remainder[(next_delim + self.delimiter.len())..];
            Some(until_delimeter)
        } else {
            self.remainder.take()
        }
    }
}

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
