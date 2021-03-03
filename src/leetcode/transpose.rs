// Problem 867
pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();

    for x in 0..matrix[0].len() {
        let mut column: Vec<i32> = Vec::new();
        for y in 0..matrix.len() {
            column.push(matrix[y][x])
        }
        result.push(column)
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_transpose() {
        assert_eq!(
            transpose(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]]
        )
    }
}
