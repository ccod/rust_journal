// Probelm 1688
pub fn number_of_matches(n: i32) -> i32 {
    let mut matches_played: i32 = 0;
    let mut players_left: i32 = n;

    while players_left != 1 {
        if players_left % 2 == 0 {
            players_left /= 2;
            matches_played += players_left;
        } else {
            matches_played += players_left / 2;
            players_left = (players_left / 2) + 1;
        }
    }

    matches_played
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_number_of_matches() {
        assert_eq!(number_of_matches(7), 6);
        assert_eq!(number_of_matches(14), 13);
    }
}
