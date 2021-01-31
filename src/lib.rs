use std::fmt::Display;

use crate::List::*;
pub mod a_star;
pub mod http_router_trie;
pub mod leetcode;
pub mod rearrange;
pub mod square_root;

pub enum List<T: Display> {
    Node { value: T, next: Box<List<T>> },
    Nil,
}

impl<T: Display> List<T> {
    pub fn new() -> List<T> {
        Nil
    }

    pub fn peek(&self) -> Option<&T> {
        match *self {
            Node { ref value, next: _ } => Some(value),
            Nil => None,
        }
    }

    pub fn prepend(self, elem: T) -> List<T> {
        Node {
            value: elem,
            next: Box::new(self),
        }
    }

    pub fn len(&self) -> i32 {
        match *self {
            Node { value: _, ref next } => 1 + next.len(),
            Nil => 0,
        }
    }

    pub fn stringify(&self) -> String {
        match *self {
            Node {
                ref value,
                ref next,
            } => {
                format!("{}, {}", value, next.stringify())
            }
            Nil => {
                format!("Nil")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_peek() {
        let mut list: List<i32> = List::new();
        list = list.prepend(12);

        match list.peek() {
            Some(value) => assert_eq!(*value, 12),
            // looking for a better way to assert failure
            None => assert_eq!(1, 2),
        }
    }

    #[test]
    fn check_len() {
        let mut list: List<i32> = List::new();

        for x in 0..200 {
            list = list.prepend(x);
        }

        assert_eq!(list.len(), 200);
    }
}
