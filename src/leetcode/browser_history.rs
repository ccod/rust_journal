// Problem 1472
struct BrowserHistory {
    history: Vec<String>,
    position: usize,
}

#[allow(dead_code)]
impl BrowserHistory {
    fn new(homepage: String) -> Self {
        BrowserHistory {
            history: vec![homepage],
            position: 0,
        }
    }

    fn visit(&mut self, page: String) {
        if self.position != self.history.len() - 1 {
            self.history.resize(self.position + 1, String::new());
        }
        self.history.push(page);
        self.position += 1
    }

    fn back(&mut self, steps: i32) -> String {
        for _ in 0..steps {
            if self.position == 0 {
                return self.history[self.position].clone();
            }
            self.position -= 1;
        }
        return self.history[self.position].clone();
    }
    fn forward(&mut self, steps: i32) -> String {
        for _ in 0..steps {
            if self.position == self.history.len() - 1 {
                return self.history[self.position].clone();
            }
            self.position += 1;
        }
        return self.history[self.position].clone();
    }
}

#[test]
fn check_browser_history() {
    let mut bhistory = BrowserHistory::new(String::from("www.leetcode.com"));
    bhistory.visit(String::from("www.google.com"));
    bhistory.visit(String::from("www.facebook.com"));
    bhistory.visit(String::from("www.youtube.com"));
    assert_eq!(bhistory.back(1), String::from("www.facebook.com"));
    assert_eq!(bhistory.back(1), String::from("www.google.com"));
    assert_eq!(bhistory.forward(1), String::from("www.facebook.com"));
}
