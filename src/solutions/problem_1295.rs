pub fn find_numbers(nums: Vec<i32>) -> i32 {
    let mut result: i32 = 0;
    for i in nums.iter() {
        if i.to_string().len() % 2 == 0 {
            result += 1;
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_find_numbers() {
        assert_eq!(find_numbers(vec![12,345,2,6,7896]), 2);
        assert_eq!(find_numbers(vec![555,901,482,1771]), 1);
    }
}
