pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
    let group: Vec<i32> = (left..=right).collect();
    let mut result: Vec<i32> = Vec::new();

    for i in group.iter() {
        let mut self_div = true;
        let as_digits: Vec<i32> = i
            .to_string()
            .chars()
            .map(|x| x.to_string().parse::<i32>().unwrap())
            .collect();
        for digit in as_digits.iter() {
            if *digit == 0 {
                self_div = false;
                break;
            }
            if *i % *digit != 0 {
                self_div = false;
                break;
            }
        }

        if self_div {
            result.push(*i)
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_self_dividing_numbers() {
        assert_eq!(self_dividing_numbers(1, 22), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22])
    }
}
