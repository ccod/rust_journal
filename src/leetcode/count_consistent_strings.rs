use std::collections::HashSet;
// Problem 1684
pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
    let mut allowed_set: HashSet<char> = HashSet::new();
    let mut count: i32 = 0;
    for c in allowed.chars() {
        allowed_set.insert(c);
    }

    for word in words.iter() {
        let mut legal = true;
        for c in word.chars() {
            if !allowed_set.contains(&c) {
                legal = false;
                break;
            }
        }

        if legal {
            count += 1
        }
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn check_count_consistent_strings() {
        assert_eq!(
            count_consistent_strings(
                "ab".to_string(),
                vec![
                    "ad".to_string(),
                    "bd".to_string(),
                    "aaab".to_string(),
                    "baa".to_string(),
                    "badab".to_string()
                ]
            ),
            2
        );
        assert_eq!(
            count_consistent_strings(
                "abc".to_string(),
                vec![
                    "a".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                    "ab".to_string(),
                    "ac".to_string(),
                    "bc".to_string(),
                    "abc".to_string()
                ]
            ),
            7
        );
        assert_eq!(
            count_consistent_strings(
                "cad".to_string(),
                vec![
                    "cc".to_string(),
                    "acd".to_string(),
                    "b".to_string(),
                    "ba".to_string(),
                    "bac".to_string(),
                    "bad".to_string(),
                    "ac".to_string(),
                    "d".to_string()
                ]
            ),
            4
        );
    }
}
