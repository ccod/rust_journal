// Problem 1470
pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::with_capacity(nums.len());
    let posn: usize = n as usize;

    for pos in posn..nums.len() {
        result.push(nums[pos - posn]);
        result.push(nums[pos]);
    }

    return result;
}

#[test]
fn check_shuffle() {
    let nums: Vec<i32> = vec![1, 1, 2, 2];
    assert_eq!(shuffle(nums, 2), vec![1, 2, 1, 2]);

    let nums2: Vec<i32> = vec![2, 5, 1, 3, 4, 7];
    assert_eq!(shuffle(nums2, 3), vec![2, 3, 5, 4, 1, 7]);
}
