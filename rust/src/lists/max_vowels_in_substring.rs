use crate::common::solution::LeetCodeTest;
use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::vec;

pub struct Solution;

impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let vowel_collection: HashSet<char> = ['a', 'e', 'i', 'o', 'u'].into_iter().collect();
        let mut max_count = 0;
        let vector_str: Vec<char> = s.chars().collect();
        // compute all the vowels in current slice first
        for i in 0..k {
            if vowel_collection.contains(&vector_str[i as usize]) {
                max_count += 1;
            }
        }

        let mut current_count = max_count; // this is the count in first time
        for j in 0..(s.len() as i32) - k {
            // consider that the slice has been moved
            // the previous element will be
            let element_skipped = vector_str.get(j as usize).unwrap();
            if (vowel_collection.contains(element_skipped) && current_count > 0) {
                current_count -= 1;
            }
            let element_added = vector_str.get((j + k) as usize).unwrap();
            if (vowel_collection.contains(element_added)) {
                current_count += 1;
            }

            max_count = max(max_count, current_count);
        }

        return max_count;
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
