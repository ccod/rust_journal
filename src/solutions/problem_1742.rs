use std::collections::HashMap;
use std::cmp::max;

pub fn count_balls(low_limit: i32, high_limit: i32) -> i32 {
    let mut box_counts: HashMap<i32, i32> = HashMap::new();
    let mut result: i32 = 0;

    for i in low_limit..=high_limit {
        let sum = i.to_string()
            .chars()
            .into_iter()
            .map(|x| x.to_string().parse::<i32>().unwrap())
            .fold(0, |a, b| a + b);

        let entry = box_counts.entry(sum).or_insert(0);
        *entry += 1;
        result = max(result, *entry);
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_count_balls() {
        assert_eq!(count_balls(1, 10), 2);
        assert_eq!(count_balls(5, 15), 2);
        assert_eq!(count_balls(19, 28), 2);
    }
}
