pub fn get_sum(a: i64, b: i64) -> i64
{
    let numbers: i64 = if b < a {
        for i in a..=b {
            println!("{i}");
        }
        (b..=a).rev().sum()
    } else {
        (a..=b).sum()
    };
    
    numbers
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_1() {

        assert_eq!(get_sum(0, 1), 1);
        assert_eq!(get_sum(1, 2), 3);
        assert_eq!(get_sum(5, -1), 14);
        assert_eq!(get_sum(505, 4), 127759);

    }
}
