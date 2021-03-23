use std::cmp::max;

// Problem 1614
pub fn max_depth(s: String) -> i32 {
    let mut result: i32 = 0;
    let mut current: i32 = 0;

    for c in s.chars() {
        match c {
            '(' => current += 1,
            ')' => {
                result = max(current, result);
                current -= 1;
            }
            _ => (),
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_max_depth() {
        assert_eq!(max_depth("(1+(2*3)+((8)/4))+1".to_string()), 3);
        assert_eq!(max_depth("(1)+((2))+(((3)))".to_string()), 3);
        assert_eq!(max_depth("1+(2*3)/(2-1)".to_string()), 1);
        assert_eq!(max_depth("1".to_string()), 0);
    }
}
