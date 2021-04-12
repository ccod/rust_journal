use std::cmp::max;

// Problem 1266
pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
    let mut result = 0;

    if points.len() == 1 {
        return result;
    }

    let mut last_point = points[0].clone();

    for i in 1..points.len() {
        let x_delta = (last_point[0] - points[i][0]).abs();
        let y_delta = (last_point[1] - points[i][1]).abs();
        result += max(x_delta, y_delta);
        last_point = points[i].clone();
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_min_time_to_visit_all_points() {
        assert_eq!(
            min_time_to_visit_all_points(vec![vec![1, 1], vec![3, 4], vec![-1, 0]]),
            7
        );
        assert_eq!(
            min_time_to_visit_all_points(vec![vec![3, 2], vec![-2, 2]]),
            5
        );
    }
}
