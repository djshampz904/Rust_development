#[cfg(test)]
mod tests {


    #[test]
    fn test_iter() {

        let v1 = vec![1, 2, 3, 4];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), Some(&4));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 3, 5, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        assert_eq!(total, 12);
    }

    #[test]
    fn iterator_closure () {
        let v1: Vec<i32> = vec![1, 2, 3];

        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

        assert_eq!(v2, vec![2, 3, 4]);
    }

}
