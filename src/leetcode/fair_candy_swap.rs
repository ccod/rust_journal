use std::collections::HashSet;

// Problem 888
pub fn fair_candy_swap(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let suma = a.iter().fold(0, |x, y| x + y);
    let sumb = b.iter().fold(0, |x, y| x + y);
    let setb: HashSet<&i32> = b.iter().clone().collect();
    let diff = (sumb - suma) / 2;

    for x in a.iter() {
        if setb.contains(&(diff + *x)) {
            return vec![*x, diff + *x];
        }
    }

    vec![suma, sumb]
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn check_fair_candy_swap() {
        assert_eq!(fair_candy_swap(vec![1, 1], vec![2, 2]), vec![1, 2]);
        assert_eq!(fair_candy_swap(vec![2, 2], vec![1, 1]), vec![2, 1]);
        assert_eq!(fair_candy_swap(vec![1, 2], vec![2, 3]), vec![1, 2]);
        assert_eq!(fair_candy_swap(vec![2], vec![1, 3]), vec![2, 3]);
    }
}
