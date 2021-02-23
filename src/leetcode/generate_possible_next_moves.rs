// Problem 293
pub fn generate_possible_next_moves(s: String) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    if s.len() > 1 {
        for i in 0..s.len() - 1 {
            match &s[i..i + 2] {
                "++" => {
                    let mut option = String::new();
                    option.push_str(&s[..i]);
                    option.push_str("--");
                    option.push_str(&s[i + 2..]);
                    result.push(option);
                }
                _ => println!("no flippable"),
            }
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn check_generate_possible_next_moves() {
        assert_eq!(
            generate_possible_next_moves("++++".to_owned()),
            vec!["--++".to_owned(), "+--+".to_owned(), "++--".to_owned()]
        )
    }
}
