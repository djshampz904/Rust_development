pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn ends_with(words: &str, ending: &str) -> bool {

    words.ends_with(ending)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_ends() {
        assert_eq!(true, ends_with("abc", "c"));
        assert_eq!(false, ends_with("strawberry", "banana"));
    }
}
