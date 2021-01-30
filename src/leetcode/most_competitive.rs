use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::collections::BinaryHeap;

// Problem 1673
struct CompetitivePair(i32, usize);

impl Ord for CompetitivePair {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for CompetitivePair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.cmp(self))
    }
}

impl PartialEq for CompetitivePair {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for CompetitivePair {}

pub fn most_competitive(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut min_heap = BinaryHeap::with_capacity(nums.len());
    let mut result: Vec<i32> = Vec::new();
    let mut lower_bound: usize = 0;

    for (pos, val) in nums.iter().enumerate() {
        min_heap.push(CompetitivePair(*val, pos));
    }

    while result.len() < k as usize {
        let upper_bound: usize = nums.len() - (k as usize - result.len());
        let next = min_heap.pop().unwrap();
        if next.1 >= lower_bound && next.1 <= upper_bound {
            result.push(next.0);
            lower_bound = next.1;
        }
    }

    return result;
}

#[ignore]
#[test]
fn check_most_competitive() {
    let nums: Vec<i32> = vec![3, 5, 2, 6];
    assert_eq!(most_competitive(nums, 2), vec![2, 6]);

    let nums2: Vec<i32> = vec![2, 4, 3, 3, 5, 4, 9, 6];
    assert_eq!(most_competitive(nums2, 4), vec![2, 3, 3, 4]);

    let nums3: Vec<i32> = vec![71, 18, 52, 29, 55, 73, 24, 42, 66, 8, 80, 2];
    assert_eq!(most_competitive(nums3, 3), vec![8, 80, 2]);
}
