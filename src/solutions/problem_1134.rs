pub fn is_armstrong(n: i32) -> bool {
    let digits: Vec<i32> = n
        .to_string()
        .chars()
        .map(|x| x.to_string().parse::<i32>().unwrap())
        .collect();
    let mut sum: i32 = 0;

    for i in digits.iter() {
        sum += i.pow(digits.len() as u32);
    }

    sum == n
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_is_armstrong() {
        assert_eq!(is_armstrong(153), true);
        assert_eq!(is_armstrong(123), false);
    }
}
