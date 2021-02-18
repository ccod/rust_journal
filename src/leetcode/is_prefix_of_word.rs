// Problem 1455
pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
    let words: Vec<&str> = sentence.split(' ').collect();
    for (pos, w) in words.iter().enumerate() {
        if search_word.len() > w.len() {
            continue;
        }
        if w[0..search_word.len()] == search_word {
            return (pos + 1) as i32;
        }
    }
    -1
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn check_is_prefix_of_word() {
        assert_eq!(
            is_prefix_of_word("i love eating burger".to_owned(), "burg".to_owned()),
            4
        );
        assert_eq!(
            is_prefix_of_word("hellohello hellohellohello".to_owned(), "ell".to_owned()),
            -1
        );
    }
}
