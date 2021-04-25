pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
    let mut result: i32 = 0;
    let mut left: usize = 0;
    let mut right: usize = mat.len() - 1;

    for row in mat.iter() {
        if left == right {
            result += row[left];
        } else {
            result += row[left];
            result += row[right];
        }

        if right == 0 {
            return result;
        }

        left += 1;
        right -= 1;
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_diagonal_sum() {
        assert_eq!(diagonal_sum(
                vec![vec![1, 2, 3],
                     vec![4, 5, 6], 
                     vec![7, 8, 9]]
                     ), 25);

        assert_eq!(diagonal_sum(
                vec![vec![1, 1, 1, 1],
                     vec![1, 1, 1, 1],
                     vec![1, 1, 1, 1],
                     vec![1, 1, 1, 1]]), 8);

        assert_eq!(diagonal_sum(vec![vec![5]]), 5);
    }
}
