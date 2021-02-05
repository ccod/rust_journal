use std::cmp::Ordering;

// Problem 34
pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut high = nums.len() as i32 - 1;
    let mut low: i32 = 0;
    let mut mid: i32;
    let mut found: Option<i32> = None;

    while low <= high {
        mid = ((high - low) / 2) + low;
        match nums[mid as usize].cmp(&target) {
            Ordering::Equal => {
                found = Some(mid);
                break;
            }
            Ordering::Less => low = mid + 1,
            Ordering::Greater => high = mid - 1,
        }
    }

    match found {
        Some(v) => {
            high = v;
            low = v;

            while low != 0 && nums[low as usize - 1] == target {
                low -= 1
            }

            while high != nums.len() as i32 - 1 && nums[high as usize + 1] == target {
                high += 1
            }

            vec![low, high]
        }
        None => vec![-1, -1],
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn search_range_test() {
        assert_eq!(search_range(vec![5, 7, 7, 8, 8, 10], 8), vec![3, 4]);
        assert_eq!(search_range(vec![5, 7, 7, 8, 8, 10], 6), vec![-1, -1]);
        assert_eq!(search_range(vec![], 0), vec![-1, -1]);
        assert_eq!(search_range(vec![1], 0), vec![-1, -1]);
        assert_eq!(search_range(vec![2, 2], 2), vec![0, 1]);
        assert_eq!(search_range(vec![2, 2], 1), vec![-1, -1]);
    }
}
