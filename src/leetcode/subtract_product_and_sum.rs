// Problem 1281
pub fn subtract_product_and_sum(n: i32) -> i32 {
    let mut temp = n;
    let mut sum: Option<i32> = None;
    let mut product: Option<i32> = None;

    while temp != 0 {
        let digit = temp % 10;
        temp /= 10;

        if let Some(x) = sum {
            let i = product.unwrap();
            sum = Some(x + digit);
            product = Some(i * digit);
        } else {
            sum = Some(digit);
            product = Some(digit);
        }
    }

    if let Some(x) = sum {
        let i = product.unwrap();
        return i - x;
    } else {
        return -1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_subtract_product_and_sum() {
        assert_eq!(subtract_product_and_sum(234), 15);
        assert_eq!(subtract_product_and_sum(4421), 21);
    }
}
