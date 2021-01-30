// Problem 1672
pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    let mut current_max: i32 = 0;
    for account in accounts.iter() {
        let mut total: i32 = 0;
        for amount in account.iter() {
            total += amount;
        }

        if current_max < total {
            current_max = total;
        }
    }

    return current_max;
}

#[test]
fn check_maximum_wealth() {
    let accounts1: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![3, 2, 1]];
    assert_eq!(maximum_wealth(accounts1), 6);

    let accounts2: Vec<Vec<i32>> = vec![vec![1, 5], vec![7, 3], vec![3, 5]];
    assert_eq!(maximum_wealth(accounts2), 10);

    let accounts3: Vec<Vec<i32>> = vec![vec![2, 8, 7], vec![7, 1, 3], vec![1, 9, 5]];
    assert_eq!(maximum_wealth(accounts3), 17);
}
