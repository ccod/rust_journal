// Problem 1486
pub fn xor_operation(n: i32, start: i32) -> i32 {
    let mut current: Option<i32> = None;

    for i in 0..n {
        let next = start + 2 * i;
        if let Some(x) = current {
            current = Some(x ^ next);
        } else {
            current = Some(next);
        }
    }

    current.unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_xor_operation() {
        assert_eq!(xor_operation(5, 0), 8);
        assert_eq!(xor_operation(4, 3), 8);
        assert_eq!(xor_operation(1, 7), 7);
        assert_eq!(xor_operation(10, 5), 2);
    }
}
