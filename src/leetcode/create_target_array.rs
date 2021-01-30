// Problem 1389
pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::with_capacity(nums.len());
    for (pos, num) in nums.iter().enumerate() {
        result.insert(index[pos] as usize, *num);
    }
    return result;
}

#[test]
fn check_create_target_array() {
    let (nums, indexes): (Vec<i32>, Vec<i32>) = (vec![0, 1, 2, 3, 4], vec![0, 1, 2, 2, 1]);
    assert_eq!(create_target_array(nums, indexes), vec![0, 4, 1, 3, 2]);
}
