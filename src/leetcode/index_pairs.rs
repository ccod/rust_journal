// iterate over the set of characters in a string
// for each character, check if it matches a start character of the word collection
// then for check against active words if the match continues or finishes,
//  if word letter fails against predicate, remove from list.
// move completed words into result set

use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum WordPartMatch {
    Success,
    Done,
    Fail,
    Continue,
}

#[derive(Debug)]
struct Word {
    start_index: usize,
    state: WordPartMatch,
    current_index: usize,
    _word: String,
}

impl Word {
    fn new(start_index: usize, word: String) -> Self {
        Word {
            start_index,
            state: match word.len() {
                1 => WordPartMatch::Success,
                _ => WordPartMatch::Continue,
            },
            current_index: 0,
            _word: word,
        }
    }

    fn next(&mut self, letter: char) -> WordPartMatch {
        match self.state {
            WordPartMatch::Fail => return self.state,
            WordPartMatch::Success => return self.state,
            WordPartMatch::Done => return self.state,
            _ => (),
        }

        if let Some(c) = self._word.chars().nth(self.current_index) {
            if letter == c {
                self.current_index += 1;
                if self.current_index == self._word.len() {
                    self.state = WordPartMatch::Success;
                }
            } else {
                self.state = WordPartMatch::Fail
            }
        };

        return self.state;
    }

    fn index_pair(&mut self) -> Vec<i32> {
        self.state = WordPartMatch::Done;
        vec![
            self.start_index as i32,
            (self.start_index + self._word.len() - 1) as i32,
        ]
    }
}

// Problem 1065
pub fn index_pairs(text: String, words: Vec<String>) -> Vec<Vec<i32>> {
    let mut word_start: HashMap<char, Vec<String>> = HashMap::new();
    let mut word_group: Vec<Word> = Vec::new();
    let mut result: Vec<Vec<i32>> = Vec::new();

    for word in words.iter() {
        if let Some(c) = word.chars().nth(0) {
            let entry = word_start.entry(c).or_insert(Vec::new());
            entry.push(word.clone());
        }
    }

    for (i, c) in text.chars().enumerate() {
        if let Some(words) = word_start.get(&c) {
            for word in words.iter() {
                word_group.push(Word::new(i, word.clone()));
            }
        }
        for word in word_group.iter_mut() {
            match word.next(c) {
                WordPartMatch::Success => result.push(word.index_pair()),
                _ => (),
            }
        }
    }
    result.sort_by(|a, b| match a[0].partial_cmp(&b[0]).unwrap() {
        Ordering::Greater => Ordering::Greater,
        Ordering::Less => Ordering::Less,
        Ordering::Equal => a[1].partial_cmp(&b[0]).unwrap(),
    });

    result
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn check_index_pairs() {
        assert_eq!(
            index_pairs(
                "thestoryofleetcodeandme".to_owned(),
                vec![
                    "story".to_owned(),
                    "fleet".to_owned(),
                    "leetcode".to_owned()
                ]
            ),
            vec![vec![3, 7], vec![9, 13], vec![10, 17]]
        );
        assert_eq!(
            index_pairs("ababa".to_owned(), vec!["aba".to_owned(), "ab".to_owned()]),
            vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4]]
        );
    }
}
