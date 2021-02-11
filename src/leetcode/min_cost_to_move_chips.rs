use std::cmp::min;

// Problem 1217
pub fn min_cost_to_move_chips(position: Vec<i32>) -> i32 {
    let mut odd_count: i32 = 0;
    let mut even_count: i32 = 0;

    for x in position.iter() {
        let rem = *x % 2;
        match rem {
            0 => even_count += 1,
            1 => odd_count += 1,
            _ => (),
        }
    }

    return min(odd_count, even_count);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn check_min_cost_to_move_chips() {
        assert_eq!(min_cost_to_move_chips(vec![1, 2, 3]), 1)
    }
}
