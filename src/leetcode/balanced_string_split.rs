// Problem 1221
pub fn balanced_string_split(s: String) -> i32 {
    let mut r_count = 0;
    let mut l_count = 0;
    let mut result = 0;

    for c in s.chars() {
        match c {
            'R' => r_count += 1,
            'L' => l_count += 1,
            _ => (),
        }

        if r_count == l_count {
            result += 1;
            l_count = 0;
            r_count = 0;
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn check_balanced_string_split() {
        assert_eq!(balanced_string_split("RLRRLLRLRL".to_owned()), 4);
        assert_eq!(balanced_string_split("RLLLLRRRLR".to_owned()), 3);
        assert_eq!(balanced_string_split("LLLLRRRR".to_owned()), 1);
        assert_eq!(balanced_string_split("RLRRRLLRLL".to_owned()), 2);
    }
}
