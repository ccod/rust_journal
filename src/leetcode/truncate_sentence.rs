// Problem 1816
pub fn truncate_sentence(s: String, k: i32) -> String {
    let mut words: Vec<&str> = s.split(' ').collect();
    words.truncate(k as usize);
    words.join(" ").to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_truncate_sentence() {
        assert_eq!(
            truncate_sentence("Hello how are you Contestant".to_string(), 4),
            "Hello how are you".to_string()
        );
        assert_eq!(
            truncate_sentence("What is the solution to this problem".to_string(), 4),
            "What is the solution".to_string()
        );
        assert_eq!(
            truncate_sentence("chopper is not a tanuki".to_string(), 5),
            "chopper is not a tanuki".to_string()
        );
    }
}
