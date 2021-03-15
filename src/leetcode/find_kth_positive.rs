// Problem 1539
pub fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
    let mut missing: i32 = 0;
    let mut arr_idx: usize = 0;
    for i in 0..(arr.len() + k as usize) {
        if arr_idx < arr.len() && arr[arr_idx] == i as i32 + 1 {
            arr_idx += 1
        } else {
            missing += 1
        }
        if missing == k {
            return i as i32 + 1;
        }
    }
    return -1;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_find_kth_positive() {
        assert_eq!(find_kth_positive(vec![2, 3, 4, 7, 11], 5), 9);
        assert_eq!(find_kth_positive(vec![1, 2, 3, 4], 2), 6);
    }
}
