use std::cmp::max;

pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
    let mut score = 0;
    for x in 0..k as usize {
        score += card_points[x];
    }

    let mut max_score = score;

    if k as usize == card_points.len() {
        return max_score;
    }

    for x in 1..(k + 1) as usize {
        score -= card_points[(k as usize) - x];
        score += card_points[card_points.len() - x];

        max_score = max(score, max_score);
    }

    return max_score;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_max_score() {
        let nums: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 1];
        let nums2: Vec<i32> = vec![2, 2, 2];
        let nums3: Vec<i32> = vec![9, 7, 7, 9, 7, 7, 9];
        let nums4: Vec<i32> = vec![1, 1000, 1];
        let nums5: Vec<i32> = vec![1, 79, 80, 1, 1, 1, 200, 1];
        let nums6: Vec<i32> = vec![96, 90, 41, 82, 39, 74, 64, 50, 30];
        let nums7: Vec<i32> = vec![
            61, 16, 51, 40, 37, 21, 96, 70, 13, 98, 28, 75, 74, 87, 68, 55, 95, 24, 46, 87,
        ];

        assert_eq!(max_score(nums, 3), 12);
        assert_eq!(max_score(nums2, 2), 4);
        assert_eq!(max_score(nums3, 7), 55);
        assert_eq!(max_score(nums4, 1), 1);
        assert_eq!(max_score(nums5, 3), 202);
        assert_eq!(max_score(nums6, 8), 536);
        assert_eq!(max_score(nums7, 19), 1129);
    }
}
