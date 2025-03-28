pub fn test_fn_once<F>(f: F) -> i32 
where F: FnOnce(i32) -> i32 
{
        f(10)
}

pub fn test_n_mut<F>(mut f: F) -> i32
where F: FnMut(i32) -> i32 {
    f(10)
}

pub fn test_not_mut<F>(f: F) -> i32
where F: Fn(i32) -> i32 {
    f(10)
}



#[cfg(test)]
mod tests {
    use super::*;
    

    #[test]
    fn test_closures() {
        let result_once = test_fn_once(|x| x + 1);
        assert_eq!(result_once, 11);
    }

    #[test]
    fn test_mute() {
        let mut count = 0;
        let result_once = test_n_mut(|x| {
            count += x;
            count
        });
        assert_eq!(result_once, 10);
    }

    #[test]
    fn test_non_mut() {
        let result_once = test_not_mut(|x| x * 2);

        assert_eq!(result_once, 20);
    }
}

