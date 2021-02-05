// Problem 34
pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    if nums.is_empty() {
        return vec![-1, -1];
    }

    if nums.len() == 1 {
        if nums[0] == target {
            return vec![0, 0];
        } else {
            return vec![-1, -1];
        }
    }

    let mut high = nums.len() - 1;
    let mut low: usize = 0;
    let mut mid: usize;
    let mut found: Option<usize> = None;

    while low <= high {
        mid = ((high - low) / 2) + low;
        if nums[mid] == target {
            found = Some(mid);
            break;
        }

        if nums[mid] < target {
            low = mid + 1;
        } else {
            if mid == 0 {
                break;
            }

            high = mid - 1;
        }
    }

    match found {
        Some(v) => {
            high = v;
            low = v;

            let mut found_high = false;
            let mut found_low = false;

            loop {
                if found_high && found_low {
                    return vec![low as i32, high as i32];
                }

                if !found_high && high + 1 == nums.len() {
                    found_high = true;
                }
                if !found_low && low == 0 {
                    found_low = true;
                }

                if !found_high && nums[high + 1] == target {
                    high += 1
                } else {
                    found_high = true;
                }

                if !found_low && nums[low - 1] == target {
                    low -= 1;
                } else {
                    found_low = true;
                }
            }
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
