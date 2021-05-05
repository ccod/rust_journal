pub fn judge_circle(moves: String) -> bool {
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    for c in moves.chars() {
        match c {
            'U' => y += 1,
            'D' => y -= 1,
            'L' => x -= 1,
            'R' => x += 1, 
            _ => (),
        }
    }

    x == 0 && y == 0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_judge_circle() {
        assert_eq!(judge_circle("UD".to_string()), true);
        assert_eq!(judge_circle("LL".to_string()), false);
        assert_eq!(judge_circle("RRDD".to_string()), false);
        assert_eq!(judge_circle("LDRRLRLUULR".to_string()), false);
    }
}
