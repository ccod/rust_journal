// Problem 1678
pub fn interpret(command: String) -> String {
    let mut tmp = String::new();
    let mut result = String::new();
    for c in command.chars() {
        tmp.push(c);
        match &tmp[..] {
            "G" => {
                result.push('G');
                tmp.clear();
            }
            "()" => {
                result.push('o');
                tmp.clear();
            }
            "(al)" => {
                result.push_str("al");
                tmp.clear();
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
    fn check_interpret() {
        assert_eq!(interpret("G()(al)".to_string()), "Goal".to_string());
        assert_eq!(
            interpret("G()()()()(al)".to_string()),
            "Gooooal".to_string()
        );
        assert_eq!(
            interpret("(al)G(al)()()G".to_string()),
            "alGalooG".to_string()
        );
    }
}
