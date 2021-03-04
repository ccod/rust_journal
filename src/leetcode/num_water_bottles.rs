// Problem 1518
pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
    let mut result = num_bottles;
    let mut accrued = num_bottles;
    while accrued >= num_exchange {
        let quot = accrued / num_exchange;
        let rem = accrued % num_exchange;

        result += quot;
        accrued = rem + quot;
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_num_water_bottles() {
        assert_eq!(num_water_bottles(9, 3), 13);
        assert_eq!(num_water_bottles(5, 5), 6);
        assert_eq!(num_water_bottles(2, 3), 2);
    }
}
