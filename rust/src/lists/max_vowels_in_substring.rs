use crate::common::solution::LeetCodeTest;
use std::cmp::max;
use std::collections::{HashMap, HashSet};

pub struct Solution;

impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let vowel_collection: HashSet<char> = ['a', 'e', 'i', 'o', 'u'].into_iter().collect();
        let mut alphabet_is_vowel_map: HashMap<char, bool> = HashMap::new();
        let mut max_vowel_count = 0;
        let mut current_vowel_count = 0;
        for c in s.chars() {
            let char_is_vowel = alphabet_is_vowel_map
                .entry(c)
                .or_insert(vowel_collection.contains(&c));
            if *char_is_vowel {
                current_vowel_count += 1;
            } else {
                max_vowel_count = max(max_vowel_count, current_vowel_count);
                current_vowel_count = 0;
            }
            if current_vowel_count == k {
                return current_vowel_count;
            }
        }
        max_vowel_count
    }
}

impl LeetCodeTest for Solution {
    fn test_self(self) {
        let case1 = Solution::max_vowels("abciiidef".to_string(), 3);
        println!("Result ----> {}", case1);
        assert!(case1 == 3);

        let case2 = Solution::max_vowels("aeiou".to_string(), 2);
        println!("Result ----> {}", case2);
        assert!(case2 == 2);

        let case3 = Solution::max_vowels("weallloveyou".to_string(), 7);
        println!("Result ----> {}", case3);
        assert!(case3 == 4);
    }
}
