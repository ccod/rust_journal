use std::collections::BTreeSet;
use std::collections::HashSet;

pub fn largest_unique_number(a: Vec<i32>) -> i32 {
    let mut group: BTreeSet<i32> = BTreeSet::new();
    let mut excluded: HashSet<i32> = HashSet::new();

    for i in a.iter() {
        if !excluded.contains(i) {
            if group.contains(i) {
                group.remove(i);
                excluded.insert(*i);
            } else {
                group.insert(*i);
            }
        }
    }

    for i in group.iter().rev() {
        return *i;
    }

    return -1;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_largest_unique_number() {
        assert_eq!(largest_unique_number(vec![5, 7, 3, 9, 4, 9, 8, 3, 1]), 8);
        assert_eq!(largest_unique_number(vec![9, 9, 8, 8]), -1);
    }
}
