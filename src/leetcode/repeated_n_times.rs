use std::collections::HashMap;

pub fn repeated_n_times(a: Vec<i32>) -> i32 {
    let mut counter: HashMap<i32, i32> = HashMap::new();
    let answer: i32 = a.len() as i32 / 2;
    for x in a.iter() {
        let entry = counter.entry(*x).or_insert(0);
        *entry += 1;

        if *entry == answer {
            return *x;
        }
    }

    return -1;
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn check_repeated_n_times() {
        assert_eq!(repeated_n_times(vec![1, 2, 3, 3]), 3);
        assert_eq!(repeated_n_times(vec![2, 1, 2, 5, 3, 2]), 2);
        assert_eq!(repeated_n_times(vec![5, 1, 5, 2, 5, 3, 5, 4]), 5);
    }
}
