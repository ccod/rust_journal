use std::cmp::Ordering;

fn next_idx(size: usize, idx: usize) -> usize {
    idx % size
}

// Problem 1652
pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    match k.cmp(&0) {
        Ordering::Equal => code.iter().map(|_| 0).collect(),
        Ordering::Greater => {
            for i in 0..code.len() {
                let mut tmp: i32 = 0;
                for j in i + 1..i + 1 + k as usize {
                    tmp += code[next_idx(code.len(), j)]
                }
                result.push(tmp);
            }
            result
        }
        Ordering::Less => {
            for i in 0..code.len() {
                let mut tmp: i32 = 0;
                let alt_k = k.abs() as usize;
                for j in i + 1..i + 1 + alt_k {
                    tmp += code[code.len() - 1 - next_idx(code.len(), j)]
                }
                result.push(tmp);
            }
            result.reverse();
            result
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_decrypt() {
        assert_eq!(decrypt(vec![5, 7, 1, 4], 3), vec![12, 10, 16, 13]);
        assert_eq!(decrypt(vec![2, 4, 9, 3], -2), vec![12, 5, 6, 13]);
    }
}
