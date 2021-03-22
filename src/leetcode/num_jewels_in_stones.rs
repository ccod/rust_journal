use std::collections::HashSet;

pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
    let mut jewel_set: HashSet<char> = HashSet::new();
    let mut result: i32 = 0;

    for c in jewels.chars() {
        jewel_set.insert(c);
    }

    for c in stones.chars() {
        if jewel_set.contains(&c) {
            result += 1;
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_num_jewels_in_stones() {
        assert_eq!(
            num_jewels_in_stones("aA".to_string(), "aAAbbbb".to_string()),
            3
        );
        assert_eq!(num_jewels_in_stones("z".to_string(), "ZZ".to_string()), 0);
    }
}
