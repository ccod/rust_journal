use std::cmp::max;

pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![0; arr.len()];
    let mut right_max: Option<i32> = None;

    for idx in (0..arr.len()).rev() {
        if let Some(i) = right_max {
            result[idx] = i;
            right_max = Some(max(arr[idx], i));
        } else {
            result[idx] = -1;
            right_max = Some(arr[idx]);
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_replace_elements() {
        assert_eq!(replace_elements(vec![17, 18, 5, 4, 6, 1]), vec![18, 6, 6, 6, 1, -1]);
        assert_eq!(replace_elements(vec![400]), vec![-1]);
    }
}
