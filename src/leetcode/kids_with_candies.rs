use std::cmp::max;

// Problem 1431
pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let mut max_candies: i32 = 0;
    let mut possible_winners: Vec<bool> = Vec::new();
    for i in candies.iter() {
        max_candies = max(max_candies, *i);
    }

    for i in candies.iter() {
        if *i + extra_candies >= max_candies {
            possible_winners.push(true);
        } else {
            possible_winners.push(false);
        }
    }

    possible_winners
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_kids_with_candies() {
        assert_eq!(
            kids_with_candies(vec![2, 3, 5, 1, 3], 3),
            vec![true, true, true, false, true]
        );
        assert_eq!(
            kids_with_candies(vec![4, 2, 1, 1, 2], 1),
            vec![true, false, false, false, false]
        );
        assert_eq!(
            kids_with_candies(vec![12, 1, 12], 1),
            vec![true, false, true]
        );
    }
}
