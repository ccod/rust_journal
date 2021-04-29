pub fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
    let mut result: i32 = 0;

    for i in 0..start_time.len() {
        if start_time[i] <= query_time && query_time <= end_time[i] {
            result += 1
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_busy_student() {
        assert_eq!(busy_student(vec![1, 2, 3], vec![3, 2, 7], 4), 1);
        assert_eq!(busy_student(vec![4], vec![4], 4), 1);
        assert_eq!(busy_student(vec![4], vec![4], 5), 0);
        assert_eq!(busy_student(vec![1, 1, 1, 1], vec![1, 3, 2, 4], 7), 0);
    }
}
