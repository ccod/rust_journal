use std::collections::HashMap;

// Problem 1365
pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
    let mut sorted_nums = nums.clone();
    sorted_nums.sort();
    let mut less_group: HashMap<i32, i32> = HashMap::new();
    let mut last_num: Option<i32> = None;
    let mut result: Vec<i32> = Vec::new();

    for (pos, i) in sorted_nums.iter().enumerate() {
        if let Some(x) = last_num {
            if *i != x {
                less_group.insert(*i, pos as i32);
                last_num = Some(*i);
            }
        } else {
            less_group.insert(*i, pos as i32);
            last_num = Some(*i);
        }
    }

    for i in nums.iter() {
        result.push(*less_group.get(i).unwrap());
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_smaller_numbers_than_current() {
        assert_eq!(
            smaller_numbers_than_current(vec![6, 5, 4, 8]),
            vec![2, 1, 0, 3]
        );
        assert_eq!(
            smaller_numbers_than_current(vec![8, 1, 2, 2, 3]),
            vec![4, 0, 1, 1, 3]
        );
        assert_eq!(
            smaller_numbers_than_current(vec![7, 7, 7, 7]),
            vec![0, 0, 0, 0]
        );
    }
}
