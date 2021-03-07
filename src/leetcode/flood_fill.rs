use std::collections::HashSet;

// Problem 733
pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
    let mut result = image.clone();
    let mut frontier: Vec<(i32, i32)> = vec![(sr, sc)];
    let source_color = image[sr as usize][sc as usize];
    let mut explored: HashSet<(i32, i32)> = HashSet::new();
    let row_len = image.len() as i32;
    let col_len = image[0].len() as i32;

    while !frontier.is_empty() {
        let (x, y) = frontier.pop().unwrap();
        if result[x as usize][y as usize] == source_color {
            result[x as usize][y as usize] = new_color;
            if x + 1 < row_len && !explored.contains(&(x + 1, y)) {
                frontier.push((x + 1, y))
            }
            if x - 1 >= 0 && !explored.contains(&(x - 1, y)) {
                frontier.push((x - 1, y))
            }
            if y + 1 < col_len && !explored.contains(&(x, y + 1)) {
                frontier.push((x, y + 1))
            }
            if y - 1 >= 0 && !explored.contains(&(x, y - 1)) {
                frontier.push((x, y - 1))
            }
        }
        explored.insert((x, y));
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn check_flood_fill() {
        assert_eq!(
            flood_fill(vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]], 1, 1, 2),
            vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]]
        )
    }
}
