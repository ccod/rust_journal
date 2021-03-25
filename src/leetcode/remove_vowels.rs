// Problem 1119
pub fn remove_vowels(s: String) -> String {
    let mut result = String::new();

    for c in s.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => (),
            _ => result.push(c),
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_remove_vowels() {
        assert_eq!(
            remove_vowels("leetcodeisacommunityforcoders".to_string()),
            "ltcdscmmntyfrcdrs".to_string()
        );
        assert_eq!(remove_vowels("aeiou".to_string()), "".to_string());
    }
}
