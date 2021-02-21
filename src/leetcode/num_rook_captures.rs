// Problem 999
pub fn num_rook_captures(arr: Vec<Vec<char>>) -> i32 {
    let mut _rook_coord: Option<(usize, usize)> = None;
    let mut count: i32 = 0;

    for (i, row) in arr.iter().enumerate() {
        for (j, piece) in row.iter().enumerate() {
            if *piece == 'R' {
                _rook_coord = Some((i, j));
                break;
            }
        }
    }
    if let Some(coord) = _rook_coord {
        // explore north
        for i in (0..coord.0).rev() {
            match arr[i][coord.1] {
                'B' => break,
                'p' => {
                    count += 1;
                    break;
                }
                _ => continue,
            }
        }
        // explore south
        for i in coord.0..8 {
            match arr[i][coord.1] {
                'B' => break,
                'p' => {
                    count += 1;
                    break;
                }
                _ => continue,
            }
        }
        // explore east
        for i in coord.1..8 {
            match arr[coord.0][i] {
                'B' => break,
                'p' => {
                    count += 1;
                    break;
                }
                _ => continue,
            }
        }
        // explore west
        for i in (0..coord.1).rev() {
            match arr[coord.0][i] {
                'B' => break,
                'p' => {
                    count += 1;
                    break;
                }
                _ => continue,
            }
        }
    }

    count
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn check_num_rook_captures() {
        assert_eq!(
            num_rook_captures(vec![
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'R', '.', '.', '.', 'p'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.']
            ]),
            3
        );
        assert_eq!(
            num_rook_captures(vec![
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', 'p', 'p', 'p', 'p', 'p', '.', '.'],
                vec!['.', 'p', 'p', 'B', 'p', 'p', '.', '.'],
                vec!['.', 'p', 'B', 'R', 'B', 'p', '.', '.'],
                vec!['.', 'p', 'p', 'B', 'p', 'p', '.', '.'],
                vec!['.', 'p', 'p', 'p', 'p', 'p', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.']
            ]),
            0
        )
    }
}
