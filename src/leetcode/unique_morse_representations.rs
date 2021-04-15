use std::collections::HashMap;
use std::collections::HashSet;

// Problem 804
pub fn unique_morse_representations(words: Vec<String>) -> i32 {
    let morse_chars = vec![
        ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..", "--",
        "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-", "-.--",
        "--..",
    ];
    let alphabet = "abcdefghijklmnopqrstuvwxyz".to_string();
    let mut char_to_morse: HashMap<char, &str> = HashMap::new();
    let mut morse_word = String::new();
    let mut uniq_morse: HashSet<String> = HashSet::new();

    for (pos, c) in alphabet.chars().enumerate() {
        char_to_morse.insert(c, morse_chars[pos]);
    }

    for word in words.iter() {
        for c in word.chars() {
            morse_word.push_str(char_to_morse.get(&c).unwrap());
        }
        uniq_morse.insert(morse_word.clone());
        morse_word.clear();
    }

    uniq_morse.len() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_unique_morse_representations() {
        assert_eq!(
            unique_morse_representations(vec![
                "gin".to_string(),
                "zen".to_string(),
                "gig".to_string(),
                "msg".to_string()
            ]),
            2
        )
    }
}
