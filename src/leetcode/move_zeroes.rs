// Problem 283
pub fn move_zeroes(nums: &mut Vec<i32>) {
    if nums.is_empty() {
        return;
    }

    let mut left: usize = 0;
    let mut right: usize = 0;

    while right < nums.len() {
        if nums[left] != 0 {
            left += 1;
            if left > right {
                right = left;
            }
            continue;
        }

        if nums[right] == 0 {
            right += 1;
            continue;
        }

        nums.swap(left, right);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_move_zeroes() {
        let mut a = vec![0, 1, 0, 3, 12];
        move_zeroes(&mut a);
        assert_eq!(a, vec![1, 3, 12, 0, 0]);
    }
}
