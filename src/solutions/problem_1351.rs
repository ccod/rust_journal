use std::cmp::Reverse;

pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
    let mut result: i32 = 0;

    for row in grid.iter() {
        result += negatives_in_row(row);
    }
    
    result
}

fn negatives_in_row(arr: &Vec<i32>) -> i32 {
    match arr.binary_search_by_key(&Reverse(-1), |&num| Reverse(num)) {
        Ok(v) => {
            let mut leftmost = v;
            while leftmost != 0 && arr[leftmost - 1] == -1 {
                leftmost -= 1;
            }
            arr[leftmost..].len() as i32
        },
        Err(v) => {
            if arr.len() == v {
                0 as i32
            } else {
                arr[v..].len() as i32
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_count_negatives() {
        assert_eq!(count_negatives(vec![
                vec![4,3,2,-1], 
                vec![3,2,1,-1],
                vec![1,1,-1,-2],
                vec![-1,-1,-2,-3]
        ]), 8);
    }

    #[test]
    fn check_negatives_in_row() {
        assert_eq!(negatives_in_row(&vec![1, 0, -1, -2]), 2);
        assert_eq!(negatives_in_row(&vec![1]), 0);
        assert_eq!(negatives_in_row(&vec![1, -1, -1, -2]), 3);
        assert_eq!(negatives_in_row(&vec![-1, -1, -2]), 3);
        assert_eq!(negatives_in_row(&vec![-2, -3, -4]), 3);
    }
}
