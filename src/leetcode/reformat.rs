use std::cmp::Ordering;

// Problem 1417
pub fn reformat(s: String) -> String {
    let mut alpha: Vec<char> = Vec::new();
    let mut nums: Vec<char> = Vec::new();

    for c in s.chars() {
        if c.is_alphabetic() {
            alpha.push(c);
        } else {
            nums.push(c)
        }
    }

    if (alpha.len() as i32 - nums.len() as i32).abs() < 2 {
        let mut result = String::new();
        match alpha.len().partial_cmp(&nums.len()).unwrap() {
            Ordering::Equal => {
                for i in 0..alpha.len() {
                    result.push(nums[i]);
                    result.push(alpha[i]);
                }
                return result;
            }
            Ordering::Greater => {
                for i in 0..nums.len() {
                    result.push(alpha[i]);
                    result.push(nums[i]);
                }
                result.push(alpha[alpha.len() - 1]);
                return result;
            }
            Ordering::Less => {
                for i in 0..alpha.len() {
                    result.push(nums[i]);
                    result.push(alpha[i]);
                }
                result.push(nums[nums.len() - 1]);
                return result;
            }
        }
    } else {
        return "".to_owned();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_reformat() {
        assert_eq!(reformat("a0b1c2".to_owned()), "0a1b2c".to_owned());
        assert_eq!(reformat("leetcode".to_owned()), "".to_owned());
        assert_eq!(reformat("1229857369".to_owned()), "".to_owned());
        assert_eq!(reformat("covid2019".to_owned()), "c2o0v1i9d".to_owned());
        assert_eq!(reformat("ab123".to_owned()), "1a2b3".to_owned());
    }
}
