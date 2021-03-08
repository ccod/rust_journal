use std::collections::HashMap;

// Problem 1624
pub fn max_length_between_equal_characters(s: String) -> i32 {
    let mut char_indices: HashMap<char, Vec<usize>> = HashMap::new();
    let mut current_max: Option<usize> = None;

    for (pos, c) in s.chars().enumerate() {
        let entry = char_indices.entry(c).or_insert(Vec::new());
        entry.push(pos);
        let l = entry.len();
        if l > 1 {
            let size: usize = entry[l - 1] - entry[0] - 1;

            if let Some(max_size) = current_max {
                if size > max_size {
                    current_max = Some(size)
                }
            } else {
                current_max = Some(size)
            }
        }
    }

    if let Some(size) = current_max {
        return size as i32;
    } else {
        return -1;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn check_max_length_between_equal_characters() {
        assert_eq!(max_length_between_equal_characters("abca".to_owned()), 2);
        assert_eq!(max_length_between_equal_characters("aa".to_owned()), 0);
        assert_eq!(max_length_between_equal_characters("cbzxy".to_owned()), -1);
        assert_eq!(max_length_between_equal_characters("cabbac".to_owned()), 4);
    }
}
