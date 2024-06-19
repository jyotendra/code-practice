use crate::common::solution::LeetCodeTest;
use std::cmp::max;

pub struct Solution;

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let word1_len = word1.chars().count();
        let word2_len = word2.chars().count();
        let mut new_word: Vec<char> = vec![];
        for i in 0..max(word1_len, word2_len) {
            if (i <= word1_len) {
                new_word.push(word1.chars().nth(i).unwrap())
            }
            if (i <= word2_len) {
                new_word.push(word2.chars().nth(i).unwrap())
            }
        }
        return new_word.iter().collect();
    }
}

impl LeetCodeTest for Solution {
    fn test_self(self) {
        let case1 = Solution::merge_alternately("abc".to_string(), "pqr".to_string());
        println!("{}", case1);
        assert!(case1 == "apbqcr".to_string());
    }
}
