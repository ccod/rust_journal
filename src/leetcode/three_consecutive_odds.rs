// Problem 1550
pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
    let mut count: i32 = 0;
    for val in arr.iter() {
        if *val % 2 == 1 {
            count += 1
        } else {
            count = 0
        }

        if count == 3 {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn check_three_consecutive_odds() {
        assert_eq!(
            three_consecutive_odds(vec![1, 2, 34, 3, 4, 5, 7, 23, 12]),
            true
        )
    }
}
