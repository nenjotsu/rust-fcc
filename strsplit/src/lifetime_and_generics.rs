#![warn(missing_debug_implementations, missing_docs)]

#[derive(Debug)]
pub struct StrSplit<'h, D> {
    remainder: Option<&'h str>,
    delimiter: D,
}

impl<'h, D> StrSplit<'h, D> {
    pub fn new(haystack: &'h str, delimiter: D) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

pub trait Delimiter {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

impl<'h, D> Iterator for StrSplit<'h, D>
where
    D: Delimiter,
{
    type Item = &'h str;

    // refactor
    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder.as_mut()?;

        if let Some((delim_start, delim_end)) = self.delimiter.find_next(remainder) {
            let until_delimeter = &remainder[..delim_start];
            *remainder = &remainder[delim_end..];
            Some(until_delimeter)
        } else {
            self.remainder.take()
        }
    }
}

impl Delimiter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self).map(|start| (start, start + self.len()))
    }
}

impl Delimiter for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.char_indices()
            .find(|(_, c)| c == self)
            .map(|(start, _)| (start, start + self.len_utf8()))
    }
}

#[allow(unused)]
fn until_char(s: &str, c: char) -> &str {
    StrSplit::new(s, c)
        .next()
        .expect("StrSplit always gives at least one result")
}

#[test]
fn test_until_chart() {
    assert_eq!(until_char("hello world", 'w'), "hello ");
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
