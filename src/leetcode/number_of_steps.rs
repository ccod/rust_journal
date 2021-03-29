// Problem 1342
pub fn number_of_steps(num: i32) -> i32 {
    let mut result: i32 = 0;
    let mut current: i32 = num;
    while current != 0 {
        if current % 2 == 0 {
            current /= 2;
            result += 1;
        } else {
            current -= 1;
            result += 1;
        }
    }

    result
}

#[cfg(tes)]
mod test {
    use super::*;

    #[test]
    fn check_number_of_steps() {
        assert_eq!(number_of_steps(14), 6);
        assert_eq!(number_of_steps(8), 4);
        assert_eq!(number_of_steps(123), 12);
    }
}
