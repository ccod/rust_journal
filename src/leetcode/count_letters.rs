// Problem 1180
pub fn count_letters(s: String) -> i32 {
    let mut prev: Option<char> = None;
    let mut substr: String = "".to_owned();
    let mut groupings: Vec<String> = Vec::new();
    let mut result: i32 = 0;

    for x in s.chars() {
        match prev {
            Some(c) => {
                if c == x {
                    substr.push(x)
                } else {
                    groupings.push(substr.clone());
                    substr.clear();
                    substr.push(x);
                    prev = Some(x);
                }
            }
            None => {
                prev = Some(x);
                substr.push(x);
            }
        }
    }

    groupings.push(substr.clone());

    for x in groupings.iter() {
        for i in 1..x.len() + 1 {
            result += i as i32
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn check_count_letters() {
        assert_eq!(count_letters("aaaba".to_owned()), 8);
        assert_eq!(count_letters("aaaaaaaaaa".to_owned()), 55);
    }
}
