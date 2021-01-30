// Problem 1656
#[allow(dead_code)]
struct OrderedStream {
    place_holder: usize,
    stream: Vec<Option<String>>,
}

#[allow(dead_code)]
impl OrderedStream {
    fn new(n: i32) -> Self {
        OrderedStream {
            place_holder: 0,
            stream: vec![None; n as usize],
        }
    }

    fn insert(&mut self, idx: i32, value: String) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        self.stream[(idx - 1) as usize] = Some(value);

        loop {
            if self.stream.len() == self.place_holder {
                break;
            }
            let item = &self.stream[self.place_holder];
            match item {
                Some(x) => {
                    result.push(x.clone());
                    self.place_holder += 1;
                }
                None => break,
            }
        }

        return result;
    }
}

#[test]
fn check_ordered_stream() {
    let mut o = OrderedStream::new(5);
    let a: Vec<String> = Vec::new();
    assert_eq!(o.insert(3, String::from("ccccc")), a);
    assert_eq!(o.insert(1, String::from("aaaaa")), vec!["aaaaa"]);
    assert_eq!(o.insert(2, String::from("bbbbb")), vec!["bbbbb", "ccccc"]);
    assert_eq!(o.insert(5, String::from("eeeee")), a);
    assert_eq!(o.insert(4, String::from("ddddd")), vec!["ddddd", "eeeee"]);
}
