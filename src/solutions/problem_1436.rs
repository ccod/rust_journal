use std::collections::HashSet;

pub fn dest_city(paths: Vec<Vec<String>>) -> String {
    let mut from: HashSet<String> = HashSet::new();
    let mut to: HashSet<String> = HashSet::new();
    let mut result = String::new();

    for path in paths.iter() {
        from.insert(path[0].clone());
        to.insert(path[1].clone());
    } 

    for x in to.difference(&from) {
        result = x.clone();
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_dest_city() {
        assert_eq!(dest_city(vec![
                vec!["London".to_string(), "New York".to_string()], 
                vec!["New York".to_string(),"Lima".to_string()],
                vec!["Lima".to_string(),"Sao Paulo".to_string()]
        ]), "Sao Paulo".to_string());
    }
}
