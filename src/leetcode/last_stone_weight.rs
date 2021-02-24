use std::cmp::Ordering;
use std::collections::BinaryHeap;

// Problem 1046
pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
    let mut ordered: BinaryHeap<i32> = BinaryHeap::from(stones);

    while ordered.len() > 1 {
        if let Some(a) = ordered.pop() {
            if let Some(b) = ordered.pop() {
                match a.cmp(&b) {
                    Ordering::Greater => ordered.push(a - b),
                    Ordering::Less => ordered.push(b - a),
                    Ordering::Equal => {
                        if ordered.len() == 0 {
                            return 0;
                        }
                    }
                }
            }
        }
    }

    return ordered.pop().unwrap();
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn check_last_stone_weight() {
        assert_eq!(last_stone_weight(vec![2, 7, 4, 1, 8, 1]), 1)
    }
}
