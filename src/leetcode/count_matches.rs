// Problem 1773
pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
    let mut result: i32 = 0;
    let key: usize = match &rule_key[..] {
        "type" => 0,
        "color" => 1,
        "name" => 2,
        _ => panic!("found something that shouldn't be here"),
    };

    for i in items.iter() {
        if rule_value == i[key] {
            result += 1
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_count_matches() {
        assert_eq!(
            count_matches(
                vec![
                    vec!["phone".to_string(), "blue".to_string(), "pixel".to_string()],
                    vec![
                        "computer".to_string(),
                        "silver".to_string(),
                        "lenovo".to_string()
                    ],
                    vec![
                        "phone".to_string(),
                        "gold".to_string(),
                        "iphone".to_string()
                    ]
                ],
                "color".to_string(),
                "silver".to_string()
            ),
            1
        );
    }
}
