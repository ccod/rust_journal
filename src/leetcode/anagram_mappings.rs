use std::collections::HashMap;

// Problem 760
pub fn anagram_mappings(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let mut at_idx: HashMap<i32, usize> = HashMap::new();
    let mut result: Vec<i32> = Vec::new();

    for (pos, i) in b.iter().enumerate() {
        at_idx.insert(*i, pos);
    }

    for i in a.iter() {
        if let Some(j) = at_idx.get(i) {
            result.push(*j as i32);
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_anagram_mappings() {
        assert_eq!(
            anagram_mappings(vec![12, 28, 46, 32, 50], vec![50, 12, 32, 46, 28]),
            vec![1, 4, 3, 2, 0]
        );
    }
}
