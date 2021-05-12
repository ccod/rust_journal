pub fn height_checker(heights: Vec<i32>) -> i32 {
    let mut sorted_heights = heights.clone();
    sorted_heights.sort();
    let mut result = 0;

    for i in 0..heights.len() {
        if heights[i] != sorted_heights[i] {
            result += 1;
        }
    }

    // println!("HELLO!! sorted: {:?}", sorted);

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_height_checker() {
        assert_eq!(height_checker(vec![1, 1, 4, 2, 1, 3]), 3);
    }
}
