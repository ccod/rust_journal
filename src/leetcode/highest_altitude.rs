use std::cmp::max;

pub fn largest_altitude(gain: Vec<i32>) -> i32 {
    let mut current_alt = 0;
    let mut current_max = 0;
    for x in gain.iter() {
        current_alt += *x;
        current_max = max(current_max, current_alt);
    }
    return current_max;
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn check_largest_altitude() {
        assert_eq!(largest_altitude(vec![-5, 1, 5, 0, -7]), 1);
        assert_eq!(largest_altitude(vec![-4, -3, -2, -1, 4, 3, 2]), 0);
    }
}
