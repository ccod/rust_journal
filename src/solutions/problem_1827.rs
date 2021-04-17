use std::cmp::Ordering;

pub fn min_operations(nums: Vec<i32>) -> i32 {
    let mut last: Option<i32> = None;
    let mut result: i32 = 0;

    for i in nums.iter() {
        if let Some(j) = last {
            match i.cmp(&j) {
                Ordering::Greater => last = Some(*i),
                Ordering::Equal => {
                    last = Some(*i + 1);
                    result += 1;
                },
                Ordering::Less => {
                    let num_incs = (j - *i) + 1;
                    result += num_incs;
                    last = Some(*i + num_incs);
                },
            }
        } else {
            last = Some(*i);
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_min_operations() {
        assert_eq!(min_operations(vec![1, 1, 1]), 3);
        assert_eq!(min_operations(vec![1,5,2,4,1]), 14);
        assert_eq!(min_operations(vec![8]), 0);
    }
}
