#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    fn append(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        Some(Box::new(ListNode { next: head, val }))
    }

    fn from(xs: Vec<i32>) -> Option<Box<ListNode>> {
        let mut current: Option<Box<ListNode>> = None;
        for i in xs.iter().rev() {
            current = ListNode::append(current, *i)
        }
        current
    }
}

// Problem 1290
pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
    let mut sum: i32 = 0;
    let mut exponent: u32 = 0;
    let base: i32 = 2;
    let mut current = head;
    let mut col: Vec<i32> = Vec::new();

    while let Some(x) = current {
        col.push(x.val);
        current = x.next;
    }

    for i in col.iter().rev() {
        if *i == 1 {
            sum += base.pow(exponent);
        }
        exponent += 1;
    }

    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_get_decimal_value() {
        let list = ListNode::from(vec![1, 0, 1]);
        assert_eq!(get_decimal_value(list), 5);
        let list = ListNode::from(vec![0]);
        assert_eq!(get_decimal_value(list), 0);
        let list = ListNode::from(vec![1]);
        assert_eq!(get_decimal_value(list), 1);
        let list = ListNode::from(vec![1, 0, 0, 1, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0]);
        assert_eq!(get_decimal_value(list), 18880);
    }
}
