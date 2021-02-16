// Problem 1064
pub fn fixed_point(arr: Vec<i32>) -> i32 {
    for (pos, x) in arr.iter().enumerate() {
        if pos as i32 == *x {
            return pos as i32;
        }
    }

    return -1;
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn check_fixed_point() {
        assert_eq!(fixed_point(vec![-10, -5, 0, 3, 7]), 3)
    }
}
