use std::collections::HashSet;

// Problem 1496
pub fn is_path_crossing(path: String) -> bool {
    let mut explored: HashSet<(i32, i32)> = HashSet::new();
    let mut current = (0, 0);
    explored.insert(current.clone());

    for c in path.chars() {
        current = match c {
            'N' => (current.0, current.1 + 1),
            'E' => (current.0 + 1, current.1),
            'W' => (current.0 - 1, current.1),
            'S' => (current.0, current.1 - 1),
            _ => current,
        };
        if explored.contains(&current) {
            return true;
        }
        explored.insert(current.clone());
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn check_is_path_crossing() {
        assert_eq!(is_path_crossing("NES".to_owned()), false);
        assert_eq!(is_path_crossing("NESWW".to_owned()), true);
        assert_eq!(is_path_crossing("SN".to_owned()), true);
    }
}
