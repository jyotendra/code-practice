// https://leetcode.com/problems/string-compression/description/?envType=study-plan-v2&envId=leetcode-75

/**
 * 443. String Compression
Medium
Hint
Given an array of characters chars, compress it using the following algorithm:

Begin with an empty string s. For each group of consecutive repeating characters in chars:

If the group's length is 1, append the character to s.
Otherwise, append the character followed by the group's length.
The compressed string s should not be returned separately, but instead, be stored in the input character array chars. Note that group lengths that are 10 or longer will be split into multiple characters in chars.

After you are done modifying the input array, return the new length of the array.

You must write an algorithm that uses only constant extra space.

Example 1:

Input: chars = ["a","a","b","b","c","c","c"]
Output: Return 6, and the first 6 characters of the input array should be: ["a","2","b","2","c","3"]
Explanation: The groups are "aa", "bb", and "ccc". This compresses to "a2b2c3".
Example 2:

Input: chars = ["a"]
Output: Return 1, and the first character of the input array should be: ["a"]
Explanation: The only group is "a", which remains uncompressed since it's a single character.
Example 3:

Input: chars = ["a","b","b","b","b","b","b","b","b","b","b","b","b"]
Output: Return 4, and the first 4 characters of the input array should be: ["a","b","1","2"].
Explanation: The groups are "a" and "bbbbbbbbbbbb". This compresses to "ab12".

 *
 */
use crate::common::solution::LeetCodeTest;

pub struct Solution;

impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        if chars.len() == 1 {
            return 1;
        }
        let mut current_repition_count = 1;
        let mut result: Vec<char> = vec![*chars.get(0).unwrap()];
        let mut i = 1;

        while i < chars.len() {
            let prev_el = chars.get(i - 1);
            let current_char = chars.get(i).unwrap();
            if let Some(prev_char) = prev_el {
                if *prev_char == *current_char {
                    current_repition_count += 1;
                } else {
                    if (current_repition_count > 1) {
                        let char_current_repition_count: Vec<char> =
                            current_repition_count.to_string().chars().collect();
                        result = [result, char_current_repition_count].concat();
                        current_repition_count = 0;
                    }
                    result.push(*current_char);
                    current_repition_count = 1;
                }
            }
            i += 1;
            if i >= chars.len() {
                let char_current_repition_count: Vec<char> =
                    current_repition_count.to_string().chars().collect();
                result = [result, char_current_repition_count].concat();
                current_repition_count = 0;
            }
        }

        println!("{:?}", result);
        // writes back to the input chars vector
        chars.clear();
        chars.extend(result.iter().cloned());
        return result.len() as i32;
    }
}

impl LeetCodeTest for Solution {
    fn test_self(self) {
        let mut test1: Vec<char> = vec!["a", "a", "b", "b", "c", "c", "c"]
            .iter()
            .flat_map(|s| s.chars())
            .collect();
        let case1 = Solution::compress(&mut test1);
        assert_eq!(case1, 6);
    }
}
