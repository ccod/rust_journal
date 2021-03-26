use std::collections::HashMap;

// Problem 1165
pub fn calculate_time(keyboard: String, word: String) -> i32 {
    let mut key_pos: HashMap<char, i32> = HashMap::new();
    let mut result: i32 = 0;
    let mut last_pos: i32 = 0;

    for (pos, c) in keyboard.chars().enumerate() {
        key_pos.insert(c, pos as i32);
    }

    for c in word.chars() {
        let current_pos = key_pos.get(&c).unwrap();
        result += (last_pos - *current_pos).abs();
        last_pos = *current_pos;
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_calculate_time() {
        assert_eq!(
            calculate_time("abcdefghijklmnopqrstuvwxyz".to_string(), "cba".to_string()),
            4
        );
        assert_eq!(
            calculate_time(
                "pqrstuvwxyzabcdefghijklmno".to_string(),
                "leetcode".to_string()
            ),
            73
        );
    }
}
