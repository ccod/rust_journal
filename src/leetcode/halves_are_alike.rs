// Problem 1704
pub fn halves_are_alike(s: String) -> bool {
    let half = s.len() / 2;
    let a: Vec<char> = s[..half].chars().collect();
    let b: Vec<char> = s[half..].chars().collect();
    let mut a_count = 0;
    let mut b_count = 0;

    for i in 0..half {
        match a[i] {
            'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => a_count += 1,
            _ => (),
        }
        match b[i] {
            'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => b_count += 1,
            _ => (),
        }
    }

    a_count == b_count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_halves_are_alike() {
        assert_eq!(halves_are_alike("book".to_string()), true);
        assert_eq!(halves_are_alike("textbook".to_string()), false);
        assert_eq!(halves_are_alike("MerryChristmas".to_string()), false);
        assert_eq!(halves_are_alike("AbCdEfGh".to_string()), true);
    }
}
