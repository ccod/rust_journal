use std::collections::HashSet;

pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
    let mut unique: HashSet<i32> = HashSet::new();
    let mut dups: HashSet<i32> = HashSet::new();
    let mut result: i32 = 0;

    for i in nums.iter() {
        if dups.contains(i) {
            continue;
        }
        if unique.contains(i) {
            unique.remove(i);
            dups.insert(*i);
        } else {
            unique.insert(*i);
        }
    }

    for i in unique.iter() {
        result += *i
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_sum_of_unique() {
        assert_eq!(sum_of_unique(vec![1, 2, 3, 2]), 4);
        assert_eq!(sum_of_unique(vec![1, 1, 1, 1]), 0);
        assert_eq!(sum_of_unique(vec![1, 2, 3, 4, 5]), 15);
    }
}
