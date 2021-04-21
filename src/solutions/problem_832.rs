pub fn flip_and_invert_image(image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();

    for row in image.iter() {
        let mut items: Vec<i32> = Vec::new();
        for item in row.iter().rev() {
            match *item {
                1 => items.push(0),
                0 => items.push(1),
                _ => (),
            }
        }
        result.push(items)
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_flip_and_invert_image() {
        assert_eq!(flip_and_invert_image(vec![vec![1,1,0], vec![1, 0, 1], vec![0,0,0]]),
            vec![vec![1,0,0], vec![0,1,0], vec![1,1,1]])
    }
}
