// Problem 1323
pub fn maximum69_number(num: i32) -> i32 {
    let tmp = num.to_string();
    let mut result = String::new();
    let mut flipped = false;

    for c in tmp.chars() {
        if !flipped {
            match c {
                '6' => {
                    result.push('9');
                    flipped = true;
                },
                _ => result.push(c),
            }
        } else {
            result.push(c);
        }
    }

    result.parse::<i32>().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_maximum69_number() {
        assert_eq!(maximum69_number(9669), 9969);
        assert_eq!(maximum69_number(9996), 9999);
        assert_eq!(maximum69_number(9999), 9999);
    }
}
