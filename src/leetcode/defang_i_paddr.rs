// Problem 1108
pub fn defang_i_paddr(address: String) -> String {
    let mut result = String::new();

    for c in address.chars() {
        match c {
            '.' => {
                result.push('[');
                result.push(c);
                result.push(']');
            }
            _ => result.push(c),
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_defang_i_paddr() {
        assert_eq!(
            defang_i_paddr("1.1.1.1".to_string()),
            "1[.]1[.]1[.]1".to_string()
        );
        assert_eq!(
            defang_i_paddr("255.100.50.0".to_string()),
            "255[.]100[.]50[.]0".to_string()
        );
    }
}
