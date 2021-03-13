// Problem 252
pub fn can_attend_meetings(intervals: Vec<Vec<i32>>) -> bool {
    let mut vals = intervals.clone();
    vals.sort_by(|a, b| a[0].partial_cmp(&b[0]).unwrap());

    for i in 1..vals.len() {
        if vals[i - 1][1] > vals[i][0] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_can_attend_meetings() {
        assert_eq!(
            can_attend_meetings(vec![vec![0, 30], vec![15, 20], vec![5, 10]]),
            false
        );
        assert_eq!(can_attend_meetings(vec![vec![7, 10], vec![2, 4]]), true);
        assert_eq!(can_attend_meetings(Vec::new()), true);
        assert_eq!(can_attend_meetings(vec![vec![2, 7]]), true);
    }
}
