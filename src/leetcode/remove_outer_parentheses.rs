// Problem 1021
pub fn remove_outer_parentheses(s: String) -> String {
    let mut result = String::new();
    let mut depth = 0;

    for c in s.chars() {
        match c {
            '(' => {
                depth += 1;
                if depth > 1 {
                    result.push(c);
                }
            }
            ')' => {
                if depth > 1 {
                    result.push(c);
                }
                depth -= 1;
            }
            _ => (),
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_remove_outer_parentheses() {
        assert_eq!(
            remove_outer_parentheses("(()())(())".to_string()),
            "()()()".to_string()
        );
        assert_eq!(
            remove_outer_parentheses("(()())(())(()(()))".to_string()),
            "()()()()(())".to_string()
        );
        assert_eq!(remove_outer_parentheses("()()".to_string()), "".to_string());
    }
}
