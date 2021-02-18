// Problem 1598
pub fn min_operations(logs: Vec<String>) -> i32 {
    let mut stack: Vec<String> = Vec::new();
    for action in logs.iter() {
        match action.as_str() {
            "../" => {
                stack.pop();
            }
            "./" => (),
            _ => stack.push(action.clone()),
        }
    }
    return stack.len() as i32;
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn check_min_operations() {
        assert_eq!(
            min_operations(vec![
                "d1/".to_owned(),
                "d2/".to_owned(),
                "../".to_owned(),
                "d21/".to_owned(),
                "./".to_owned()
            ]),
            2
        )
    }
}
