use std::cmp::Ordering;

// Problem 1471
pub fn get_strongest(arr: Vec<i32>, k: i32) -> Vec<i32> {
    let mut temp: Vec<i32> = arr;
    temp.sort();

    let median: i32 = temp[((temp.len() - 1) / 2)];

    temp.sort_by(|a, b| {
        let result = (a - median).abs().partial_cmp(&(b - median).abs()).unwrap();
        return match result {
            Ordering::Greater => Ordering::Less,
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => {
                if a > b {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }
        };
    });

    temp.resize(k as usize, 0);
    return temp;
}

#[test]
fn check_get_strongest() {
    let nums: Vec<i32> = vec![1, 2, 3, 4, 5];
    let nums2: Vec<i32> = vec![1, 1, 3, 5, 5];
    let nums3: Vec<i32> = vec![6, 7, 11, 7, 6, 8];
    let nums4: Vec<i32> = vec![6, -3, 7, 2, 11];
    let nums5: Vec<i32> = vec![-7, 22, 17, 3];

    assert_eq!(get_strongest(nums, 2), vec![5, 1]);
    assert_eq!(get_strongest(nums2, 2), vec![5, 5]);
    assert_eq!(get_strongest(nums3, 5), vec![11, 8, 6, 6, 7]);
    assert_eq!(get_strongest(nums4, 3), vec![-3, 11, 2]);
    assert_eq!(get_strongest(nums5, 2), vec![22, 17]);
}
