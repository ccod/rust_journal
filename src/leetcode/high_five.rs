use std::collections::BTreeMap;
use std::collections::BinaryHeap;

// Problem 1086
pub fn high_five(items: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut collection: BTreeMap<i32, BinaryHeap<i32>> = BTreeMap::new();
    let mut result: Vec<Vec<i32>> = Vec::new();
    for grade in items.iter() {
        let entry = collection.entry(grade[0]).or_insert(BinaryHeap::new());
        entry.push(grade[1]);
    }
    for (id, grades) in &mut collection {
        let mut avrg: i32 = 0;
        for _ in 0..5 {
            avrg += match grades.pop() {
                Some(v) => v,
                None => 0,
            }
        }
        result.push(vec![*id, avrg / 5]);
    }
    return result;
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn check_high_five() {
        assert_eq!(
            high_five(vec![
                vec![1, 91],
                vec![1, 92],
                vec![2, 93],
                vec![2, 97],
                vec![1, 60],
                vec![2, 77],
                vec![1, 65],
                vec![1, 87],
                vec![1, 100],
                vec![2, 100],
                vec![2, 76]
            ]),
            vec![vec![1, 87], vec![2, 88]]
        );
    }
}
