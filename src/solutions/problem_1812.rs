pub fn square_is_white(coordinates: String) -> bool {
    let coords: Vec<char> = coordinates.chars().collect();

    let column = match coords[0] {
        'a' | 'c' | 'e' | 'g' => true,
        'b' | 'd' | 'f' | 'h' => false,
        _ => false,
    };

    let row = coords[1].to_string().parse::<i32>().unwrap();

    if row % 2 == 0 {
        column
    } else {
        !column
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_square_is_white() {
        assert_eq!(square_is_white("a1".to_string()), false);
        assert_eq!(square_is_white("h3".to_string()), true);
        assert_eq!(square_is_white("c7".to_string()), false);
    }
}
