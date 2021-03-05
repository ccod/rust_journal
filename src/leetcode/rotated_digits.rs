// Problem 788
pub fn rotated_digits(n: i32) -> i32 {
    let mut result = 0;
    for x in 1..=n {
        let mut left = x;
        let mut valid = true;
        let mut mult = 1;
        let mut new_num = 0;

        while left != 0 {
            let current = left % 10;
            left /= 10;

            let tmp = match current {
                2 => 5,
                5 => 2,
                6 => 9,
                9 => 6,
                0 => 0,
                1 => 1,
                8 => 8,
                _ => {
                    valid = false;
                    break;
                }
            };
            new_num += mult * tmp;
            mult *= 10;
        }
        if new_num == x {
            valid = false;
        }
        if valid {
            result += 1;
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn check_rotated_digits() {
        assert_eq!(rotated_digits(10), 4);
        assert_eq!(rotated_digits(857), 247);
    }
}
