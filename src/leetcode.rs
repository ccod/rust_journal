use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::collections::{BinaryHeap, HashMap};

// Problem 1672
pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    let mut current_max: i32 = 0;
    for account in accounts.iter() {
        let mut total: i32 = 0;
        for amount in account.iter() {
            total += amount;
        }

        if current_max < total {
            current_max = total;
        }
    }

    return current_max;
}

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

// Problem 1470
pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::with_capacity(nums.len());
    let posn: usize = n as usize;

    for pos in posn..nums.len() {
        result.push(nums[pos - posn]);
        result.push(nums[pos]);
    }

    return result;
}

// Problem 1471
pub fn get_strongest(arr: Vec<i32>, k: i32) -> Vec<i32> {
    let mut temp: Vec<i32> = arr;
    temp.sort();

    let median: i32 = temp[((temp.len() - 1) / 2)];

    temp.sort_by(|a, b| {
        let result = (a - median).abs().partial_cmp(&(b - median).abs()).unwrap();
        return match result {
            Ordering::Greater => Ordering::Less,
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => {
                if a > b {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }
        };
    });

    temp.resize(k as usize, 0);
    return temp;
}

// Problem 1512
pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
    let mut counter: i32 = 0;
    let mut value_groupings = HashMap::new();

    for i in nums.iter() {
        value_groupings
            .entry(i)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }

    for c in value_groupings.values() {
        for i in 0..*c {
            counter += i
        }
    }

    return counter;
}

// Problem 1472
struct BrowserHistory {
    history: Vec<String>,
    position: usize,
}

#[allow(dead_code)]
impl BrowserHistory {
    fn new(homepage: String) -> Self {
        BrowserHistory {
            history: vec![homepage],
            position: 0,
        }
    }

    fn visit(&mut self, page: String) {
        if self.position != self.history.len() - 1 {
            self.history.resize(self.position + 1, String::new());
        }
        self.history.push(page);
        self.position += 1
    }

    fn back(&mut self, steps: i32) -> String {
        for _ in 0..steps {
            if self.position == 0 {
                return self.history[self.position].clone();
            }
            self.position -= 1;
        }
        return self.history[self.position].clone();
    }
    fn forward(&mut self, steps: i32) -> String {
        for _ in 0..steps {
            if self.position == self.history.len() - 1 {
                return self.history[self.position].clone();
            }
            self.position += 1;
        }
        return self.history[self.position].clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_maximum_wealth() {
        let accounts1: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![3, 2, 1]];
        assert_eq!(maximum_wealth(accounts1), 6);

        let accounts2: Vec<Vec<i32>> = vec![vec![1, 5], vec![7, 3], vec![3, 5]];
        assert_eq!(maximum_wealth(accounts2), 10);

        let accounts3: Vec<Vec<i32>> = vec![vec![2, 8, 7], vec![7, 1, 3], vec![1, 9, 5]];
        assert_eq!(maximum_wealth(accounts3), 17);
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

    #[test]
    fn check_shuffle() {
        let nums: Vec<i32> = vec![1, 1, 2, 2];
        assert_eq!(shuffle(nums, 2), vec![1, 2, 1, 2]);

        let nums2: Vec<i32> = vec![2, 5, 1, 3, 4, 7];
        assert_eq!(shuffle(nums2, 3), vec![2, 3, 5, 4, 1, 7]);
    }

    #[test]
    fn check_get_strongest() {
        let nums: Vec<i32> = vec![1, 2, 3, 4, 5];
        let nums2: Vec<i32> = vec![1, 1, 3, 5, 5];
        let nums3: Vec<i32> = vec![6, 7, 11, 7, 6, 8];
        let nums4: Vec<i32> = vec![6, -3, 7, 2, 11];
        let nums5: Vec<i32> = vec![-7, 22, 17, 3];

        assert_eq!(get_strongest(nums, 2), vec![5, 1]);
        assert_eq!(get_strongest(nums2, 2), vec![5, 5]);
        assert_eq!(get_strongest(nums3, 5), vec![11, 8, 6, 6, 7]);
        assert_eq!(get_strongest(nums4, 3), vec![-3, 11, 2]);
        assert_eq!(get_strongest(nums5, 2), vec![22, 17]);
    }

    #[test]
    fn check_num_identical_pairs() {
        let nums: Vec<i32> = vec![1, 2, 3, 1, 1, 3];
        let nums2: Vec<i32> = vec![1, 1, 1, 1];
        let nums3: Vec<i32> = vec![1, 2, 3];

        assert_eq!(num_identical_pairs(nums), 4);
        assert_eq!(num_identical_pairs(nums2), 6);
        assert_eq!(num_identical_pairs(nums3), 0);
    }

    #[test]
    fn check_browser_history() {
        let mut bhistory = BrowserHistory::new(String::from("www.leetcode.com"));
        bhistory.visit(String::from("www.google.com"));
        bhistory.visit(String::from("www.facebook.com"));
        bhistory.visit(String::from("www.youtube.com"));
        assert_eq!(bhistory.back(1), String::from("www.facebook.com"));
        assert_eq!(bhistory.back(1), String::from("www.google.com"));
        assert_eq!(bhistory.forward(1), String::from("www.facebook.com"));
    }
}
