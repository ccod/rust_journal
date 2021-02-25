use std::collections::BTreeMap;

fn count_chars(group: &mut BTreeMap<char, i32>, s: String) {
    for c in s.chars() {
        let entry = group.entry(c).or_insert(0);
        *entry += 1
    }
}

// Problem 242
pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let mut s_counter: BTreeMap<char, i32> = BTreeMap::new();
    let mut t_counter: BTreeMap<char, i32> = BTreeMap::new();

    count_chars(&mut s_counter, s);
    count_chars(&mut t_counter, t);

    let sk: Vec<char> = s_counter.keys().cloned().collect();
    let tk: Vec<char> = t_counter.keys().cloned().collect();

    if sk != tk {
        return false;
    }

    for (c, i) in &s_counter {
        if let Some(j) = t_counter.get(c) {
            if i != j {
                return false;
            }
        }
    }

    true
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn check_is_anagram() {
        assert_eq!(is_anagram("anagram".to_owned(), "nagaram".to_owned()), true)
    }
}
