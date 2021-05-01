use std::cmp::max;

pub fn merge_alternately(word1: String, word2: String) -> String {
    let mut result = String::new();
    let chars1: Vec<char> = word1.chars().collect();
    let chars2: Vec<char> = word2.chars().collect();
    let idx = max(chars1.len(), chars2.len());

    for i in 0..idx {
        if i < chars1.len() {
            result.push(chars1[i]);
        }

        if i < chars2.len() {
            result.push(chars2[i]);
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_merge_alternately() {
        assert_eq!(merge_alternately("abcd".to_string(), "pq".to_string()), "apbqcd".to_string());
    }
}
