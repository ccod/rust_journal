use std::collections::HashMap;

// Problem 389
pub fn find_the_difference(s: String, t: String) -> char {
    let mut char_count: HashMap<char, i32> = HashMap::new();

    for c in s.chars() {
        let entry = char_count.entry(c).or_insert(0);
        *entry += 1;
    }

    for c in t.chars() {
        let entry = char_count.entry(c).or_insert(0);
        *entry -= 1;
        if *entry < 0 {
            return c;
        }
    }
    return 'q';
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn check_find_the_difference() {
        assert_eq!(
            find_the_difference("abcd".to_owned(), "abcde".to_owned()),
            'e'
        );
        assert_eq!(find_the_difference("".to_owned(), "y".to_owned()), 'y');
        assert_eq!(find_the_difference("a".to_owned(), "aa".to_owned()), 'a');
        assert_eq!(find_the_difference("ae".to_owned(), "aea".to_owned()), 'a');
    }
}
