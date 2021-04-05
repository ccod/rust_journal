// Problem 1528
pub fn restore_string(s: String, indices: Vec<i32>) -> String {
    let mut result = String::new();
    let mut temp: Vec<(i32, char)> = Vec::new();

    for (pos, c) in s.chars().enumerate() {
        temp.push((indices[pos], c));
    }

    temp.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    for (_, c) in temp.iter() {
        result.push(*c);
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_restore_string() {
        assert_eq!(
            restore_string("codeleet".to_string(), vec![4, 5, 6, 7, 0, 2, 1, 3]),
            "leetcode".to_string()
        );
        assert_eq!(
            restore_string("abc".to_string(), vec![0, 1, 2]),
            "abc".to_string()
        );
        assert_eq!(
            restore_string("aiohn".to_string(), vec![3, 1, 4, 2, 0]),
            "nihao".to_string()
        );
        assert_eq!(
            restore_string("aaiougrt".to_string(), vec![4, 0, 2, 6, 7, 3, 1, 5]),
            "arigatou".to_string()
        );
        assert_eq!(
            restore_string("art".to_string(), vec![1, 0, 2]),
            "rat".to_string()
        );
    }
}
