pub fn collinearity(x1: i32, y1: i32, x2: i32, y2: i32) -> bool
{
    match (x1, y1, x2, y2) {
        (0, _, 0, _) => {
            println!("matched (0,_,0,_)");
            return true
        },
        (_, 0, _, 0) => {
            println!("matched (_,0,0,_)");
            return true
        },
        (0, 0, _, _) => {
            println!("matched (0,0,_,_)");
            return true
        },
        (_, _, 0, 0) =>{
            println!("matched (_,_,0,0)");
            return true
        },
        (0, _, _, _) => {
            println!("matched (0,_,_,_)");
            return false 
        }
        (_, 0, _, _) => {
            println!("matched (_,0,_,_)");
            return false
        },
        _ => {
            let k1: f32 = (x2 as f32) / (x1 as f32);
            let k2: f32 = (y2 as f32) / (y1 as f32);
            

            println!("k1: {k1}, k2: {k2}");

            if k2 == k1 {
                return true
            } else {
                return false
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn do_test(x1: i32, y1: i32, x2: i32, y2: i32, expected: bool)
    {
        assert_eq!(collinearity(x1, y1, x2, y2), expected, "Input {x1},{y1},{x2},{y2}");
    }



    #[test]
    fn test_fixed_one_direction() {
        do_test(1, 1, 1, 1, true);
        do_test(1, 2, 2, 4, true);
        do_test(1, 2, 3, 7, false);
    }

    
    #[test]
    fn test_fixed_opposite_direction() {
        do_test(1, 1, 6, 1, false);
        do_test(1, 2, -1, -2, true);
        do_test(1, 2, 1, -2, false);
    }

    #[test]
    fn test_fixed_vectors_contain_zero() {
        do_test(4, 0, 11, 0, true);
        do_test(0, 1, 6, 0, false);
        do_test(4, 4, 0, 4, false);
    }
}
