use std::cmp::Reverse;
use std::collections::BinaryHeap;

// Problem 1619
pub fn trim_mean(arr: Vec<i32>) -> f64 {
    if arr.is_empty() {
        return 0 as f64;
    }

    let mut top: BinaryHeap<i32> = BinaryHeap::new();
    let mut bottom: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    let mut total: i32 = 0;

    for x in arr.iter() {
        top.push(*x);
        bottom.push(Reverse(*x));
        total += *x
    }

    let five_percent = arr.len() / 20;
    for _ in 0..five_percent {
        if let Some(x) = top.pop() {
            total -= x
        }
        if let Some(Reverse(x)) = bottom.pop() {
            total -= x
        }
    }
    let leftover = arr.len() - (five_percent * 2);
    return total as f64 / leftover as f64;
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn check_trim_mean() {
        assert_eq!(
            trim_mean(vec![
                1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3
            ]),
            2.00000
        )
    }
}
