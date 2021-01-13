pub fn square_root(number: i32) -> Result<i32, &'static str> {
    if number < 0 {
        return Err("Cannot square root negative numbers");
    }

    let num = number as f32;
    let mut guess: f32 = 1.28;

    loop {
        let next_guess: f32 = 0.5 * (guess + (num / guess));

        // this was guidence from clippy, rather than using:
        // if guess.floor() == next_guess.floor() {
        if (guess.floor() - next_guess.floor()).abs() < f32::EPSILON {
            return Ok(next_guess as i32);
        }

        guess = next_guess;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn normal_square() {
        assert_eq!(square_root(4).unwrap(), 2);
    }
}
