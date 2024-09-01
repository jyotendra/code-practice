use crate::common::solution::LeetCodeTest;

pub struct Solution;

impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        if str1.len() < str2.len() {
            return Solution::gcd_of_strings(str2, str1);
        }
        if str2.is_empty() {
            return str1;
        }
        if !str1.starts_with(&str2) {
            return "".to_string();
        }
        Solution::gcd_of_strings(str1[str2.len()..].to_string(), str2)
    }
}

impl LeetCodeTest for Solution {
    fn test_self(self) {
        let case2 = Solution::gcd_of_strings("AA".to_string(), "A".to_string());
        assert_eq!(case2, "A".to_string());
    }
}
