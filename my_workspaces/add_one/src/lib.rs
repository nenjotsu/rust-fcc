use rand;

pub fn add_one(a: i32) -> i32 {
    a + 1
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
