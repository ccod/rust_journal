pub fn sum_base(n: i32, k: i32) -> i32 {
    let mut temp: Vec<i32> = vec![0];

    for _ in 0..n {
        inc_base(&mut temp, k);
    }

    temp.iter().fold(0, |a, b| a + b)
}

fn inc_base(val: &mut Vec<i32>, base: i32) {
    let mut current_idx: usize = 0;
    let mut uncarried = true;
    while uncarried {
        if val[current_idx] + 1 < base {
            val[current_idx] += 1;
            uncarried = false;
        } else {
            val[current_idx] = 0;
            current_idx += 1;
            if val.len() == current_idx {
                val.push(1);
                uncarried = false;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_sum_base() {
        assert_eq!(sum_base(34, 6), 9);
        assert_eq!(sum_base(10, 10), 1)
    }
}
