use std::collections::BinaryHeap;
use std::cmp::min;

pub fn array_pair_sum(nums: Vec<i32>) -> i32 {
    let mut heap: BinaryHeap<i32> = BinaryHeap::new();
    let mut result = 0;

    for i in nums.iter() {
        heap.push(*i);
    }

    loop {
        if heap.peek() == None {
            break;
        }

        let a = heap.pop().unwrap();
        let b = heap.pop().unwrap();

        result += min(a, b);
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_array_pair_sum() {
        assert_eq!(array_pair_sum(vec![1, 4, 3, 2]), 4);
        assert_eq!(array_pair_sum(vec![6, 2, 6, 5, 1, 2]), 9);
    }
}
