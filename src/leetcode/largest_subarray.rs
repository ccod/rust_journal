use std::cmp::Ordering;
use std::i32::MIN;

// Problem 1708
pub fn largest_subarray(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut largest: i32 = MIN;
    let mut occurrances: Vec<usize> = Vec::new();

    for x in 0..nums.len() - (k - 1) as usize {
        match nums[x].cmp(&largest) {
            Ordering::Greater => {
                largest = nums[x];
                occurrances.clear();
                occurrances.push(x);
            }
            Ordering::Equal => {
                occurrances.push(x);
            }
            _ => (),
        }
    }

    if occurrances.len() == 1 {
        let idx = occurrances[0];
        return Vec::from(&nums[idx..idx + k as usize]);
    }

    for x in 1..k as usize {
        if occurrances.len() == 1 {
            let idx = occurrances[0];
            return Vec::from(&nums[idx..idx + k as usize]);
        }
        largest = MIN;
        let foo: Vec<(usize, i32)> = occurrances
            .iter()
            .map(|idx| (*idx, nums[idx + x]))
            .collect();
        occurrances.clear();
        for (idx, i) in foo.iter() {
            match i.cmp(&largest) {
                Ordering::Greater => {
                    occurrances.clear();
                    occurrances.push(*idx)
                }
                Ordering::Equal => occurrances.push(*idx),
                _ => (),
            }
        }
    }

    vec![]
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn check_largest_subarray() {
        assert_eq!(largest_subarray(vec![1, 4, 5, 2, 3], 3), vec![5, 2, 3]);
    }
}
