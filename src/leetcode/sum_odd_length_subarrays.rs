// Problem 1588
pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
    let mut result: i32 = 0;
    for i in 1..arr.len() + 1 {
        if i % 2 == 0 {
            continue;
        }
        for (pos, _) in arr.iter().enumerate() {
            if pos + i > arr.len() {
                break;
            }
            result += &arr[pos..pos + i].iter().fold(0, |acc, v| acc + v)
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn check_sub_odd_length_subarray() {
        assert_eq!(sum_odd_length_subarrays(vec![1, 4, 2, 5, 3]), 58);
        assert_eq!(sum_odd_length_subarrays(vec![1, 2]), 3);
        assert_eq!(sum_odd_length_subarrays(vec![10, 11, 12]), 66);
    }
}
