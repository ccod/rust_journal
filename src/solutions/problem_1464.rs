use std::collections::BinaryHeap;

pub fn max_product(nums: Vec<i32>) -> i32 {
    let mut sorted: BinaryHeap<i32> = BinaryHeap::new();
    for i in nums.iter() {
        sorted.push(*i);
    }

    let a = sorted.pop().unwrap();
    let b = sorted.pop().unwrap();

    (a - 1) * (b - 1)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_max_product() {
        assert_eq!(max_product(vec![3, 4, 5, 2]), 12);
        assert_eq!(max_product(vec![1, 5, 4, 5]), 16);
        assert_eq!(max_product(vec![3, 7]), 12);
    }
}
