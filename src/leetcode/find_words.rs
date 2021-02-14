use std::collections::HashSet;

#[derive(PartialEq, Eq, Clone)]
enum Row {
    Top,
    Middle,
    Bottom,
}

fn to_row(character: &char, predicates: &Vec<(HashSet<char>, Row)>) -> Option<Row> {
    let mut c = character.clone();
    if c.is_uppercase() {
        c.make_ascii_lowercase()
    }

    for (set, row) in predicates.iter() {
        if set.contains(&c) {
            return Some(row.clone());
        }
    }
    return None;
}

// Problem 500
pub fn find_words(words: Vec<String>) -> Vec<String> {
    let top: HashSet<char> = "qwertyuiop".chars().into_iter().collect();
    let middle: HashSet<char> = "asdfghjkl".chars().into_iter().collect();
    let bottom: HashSet<char> = "zxcvbnm".chars().into_iter().collect();
    let mut result: Vec<String> = Vec::new();

    let predicates = vec![
        (top, Row::Top),
        (middle, Row::Middle),
        (bottom, Row::Bottom),
    ];

    for word in words.iter() {
        let mut row: Option<Row> = None;
        let mut valid: bool = true;

        for c in word.chars() {
            match row {
                Some(_) => {
                    if row != to_row(&c, &predicates) {
                        valid = false
                    }
                }
                None => row = to_row(&c, &predicates),
            }
        }

        if valid {
            result.push(word.clone())
        }
    }
    return result;
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn check_find_words() {
        assert_eq!(
            find_words(vec![
                "Hello".to_owned(),
                "Alaska".to_owned(),
                "Dad".to_owned(),
                "Peace".to_owned()
            ]),
            vec!["Alaska".to_owned(), "Dad".to_owned()]
        );
        assert_eq!(
            find_words(vec![
                "Aasdfghjkl".to_owned(),
                "Qwertyuiop".to_owned(),
                "zZxcvbnm".to_owned()
            ]),
            vec![
                "Aasdfghjkl".to_owned(),
                "Qwertyuiop".to_owned(),
                "zZxcvbnm".to_owned()
            ]
        )
    }
}
