// Problem 1480
pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let mut current: i32 = 0;
    for i in nums.iter() {
        current += *i;
        result.push(current);
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_running_sum() {
        assert_eq!(running_sum(vec![1, 2, 3, 4]), vec![1, 3, 6, 10]);
        assert_eq!(running_sum(vec![1, 1, 1, 1, 1]), vec![1, 2, 3, 4, 5]);
        assert_eq!(running_sum(vec![3, 1, 2, 10, 1]), vec![3, 4, 6, 16, 17]);
    }
}
