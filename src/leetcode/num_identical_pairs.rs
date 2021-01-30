use std::collections::HashMap;

// Problem 1512
pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
    let mut counter: i32 = 0;
    let mut value_groupings = HashMap::new();

    for i in nums.iter() {
        value_groupings
            .entry(i)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }

    for c in value_groupings.values() {
        for i in 0..*c {
            counter += i
        }
    }

    return counter;
}

#[test]
fn check_num_identical_pairs() {
    let nums: Vec<i32> = vec![1, 2, 3, 1, 1, 3];
    let nums2: Vec<i32> = vec![1, 1, 1, 1];
    let nums3: Vec<i32> = vec![1, 2, 3];

    assert_eq!(num_identical_pairs(nums), 4);
    assert_eq!(num_identical_pairs(nums2), 6);
    assert_eq!(num_identical_pairs(nums3), 0);
}
