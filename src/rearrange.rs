// array as arguement
pub fn quicksort(input_list: &mut [i32], low: usize, high: usize) {
    let mut checker: usize = low;
    let mut pivot: usize = high;

    while checker + 1 != pivot {
        if input_list[checker] < input_list[pivot] {
            input_list.swap(pivot, pivot - 1);
            input_list.swap(checker, pivot);
            pivot -= 1;
        } else {
            checker += 1;
        }
    }

    if input_list[checker] < input_list[pivot] {
        input_list.swap(checker, pivot);
        pivot -= 1;
    }

    if pivot + 1 < high {
        quicksort(input_list, pivot + 1, high);
    }

    // translating quicksort, usize can't be a negative number
    if pivot > 0 && pivot - 1 > low {
        quicksort(input_list, low, pivot - 1);
    }
}

pub fn rearrange_digits(input_list: &mut [i32]) -> (i32, i32) {
    let mut first: String = "".to_owned();
    let mut second: String = "".to_owned();

    quicksort(input_list, 0, input_list.len() - 1);
    for (pos, i) in input_list.iter().enumerate() {
        if pos % 2 == 0 {
            first.push_str(&i.to_string());
        } else {
            second.push_str(&i.to_string());
        }
    }

    return (
        first.parse::<i32>().unwrap(),
        second.parse::<i32>().unwrap(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_quicksort() {
        let mut numbers: [i32; 4] = [5, 3, 2, 8];
        quicksort(&mut numbers, 0, 3);
        assert_eq!(numbers, [8, 5, 3, 2]);
    }

    #[test]
    fn check_rearrange_digits() {
        let mut numbers: [i32; 6] = [4, 6, 2, 5, 9, 8];
        let pair: (i32, i32) = rearrange_digits(&mut numbers);
        assert_eq!(pair, (964, 852));
    }
}
