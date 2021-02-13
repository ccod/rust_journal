#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[allow(dead_code)]
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    fn push(val: i32, node: Option<Box<ListNode>>) -> Option<Box<Self>> {
        Some(Box::new(ListNode { val, next: node }))
    }

    fn from_vec(mut nums: Vec<i32>) -> Option<Box<Self>> {
        let mut current: Option<Box<ListNode>> = None;
        nums.reverse();
        for x in nums.iter() {
            current = ListNode::push(*x, current);
        }
        current
    }

    fn to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut current = head.clone();
        let mut result: Vec<i32> = Vec::new();
        while current.is_some() {
            result.push(current.as_ref().unwrap().val.clone());
            current = current.unwrap().next;
        }

        result
    }
}

// Problem 876
pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut fast = head.clone();
    let mut slow = head.clone();

    while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
        fast = fast.unwrap().next.unwrap().next;
        slow = slow.unwrap().next;
    }

    slow
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_middle_node() {
        let answer = middle_node(ListNode::from_vec(vec![1, 2, 3, 4, 5]));
        assert_eq!(ListNode::to_vec(answer), vec![3, 4, 5]);
    }
}
