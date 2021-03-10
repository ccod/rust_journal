use std::collections::HashSet;

// Problem 268
pub fn missing_number(nums: Vec<i32>) -> i32 {
    let mut set: HashSet<i32> = HashSet::with_capacity(nums.len() + 1);
    let mut result: i32 = -1;
    for i in 0..=nums.len() {
        set.insert(i as i32);
    }

    for i in nums.iter() {
        set.remove(i);
    }

    for i in &set {
        result = *i;
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_missing_number() {
        assert_eq!(missing_number(vec![3, 0, 1]), 2)
    }
}
