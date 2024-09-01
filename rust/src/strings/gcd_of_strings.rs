use std::iter;

use crate::common::solution::LeetCodeTest;

pub struct Solution;

impl Solution {
    fn gcd_recursion(divisor: Option<u32>, dividend: Option<u32>) -> Option<u32> {
        if divisor.unwrap() == 0 {
            return dividend;
        }

        let remainder = dividend.unwrap() % divisor.unwrap();

        if remainder == 0 {
            return divisor;
        }

        if remainder == 1 {
            return None;
        } else {
            return Solution::gcd_recursion(Some(remainder), divisor);
        }
    }

    pub fn find_gcd(num1: u32, num2: u32) -> u32 {
        let big_num: u32;
        let small_num: u32;
        let gcd: Option<u32>;

        if num1 > num2 {
            big_num = num1;
            small_num = num2;
        } else {
            big_num = num2;
            small_num = num1;
        }

        gcd = Solution::gcd_recursion(Some(small_num), Some(big_num));

        if gcd.is_some() && gcd.unwrap() != 0 && big_num % gcd.unwrap() == 0 {
            return gcd.unwrap();
        }

        1
    }

    pub fn check_val(small: &str, long: &str, gcd: usize) -> String {
        let default_str = "".to_string();
        let comparator_str = small.get(0..gcd).unwrap();
        // check small str
        let mut i = 0;
        while i < small.len() - 1 {
            if small.get(i..i + gcd).unwrap() == comparator_str {
                i += gcd;
                continue;
            } else {
                return default_str;
            }
        }

        let mut j = 0;
        while j < long.len() - 1 {
            if long.get(j..j + gcd).unwrap() == comparator_str {
                j += gcd;
                continue;
            } else {
                return default_str;
            }
        }

        return small.get(0..gcd).unwrap().to_string();
    }

    /**
     * Dividend = Divisor * Quotient + Remainder (where divisor is length on smallest str and dividend is len of longest str)
     * If remainder == 0 then divisor, is the GCD
     * If remainder != 0, but remainder completely divides both dividend and divisor, then remainder is the GCD.
     */
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        let smallest_str: &str;
        let longest_str: &str;
        if str1.len() < str2.len() {
            smallest_str = &str1;
            longest_str = &str2;
        } else {
            smallest_str = &str2;
            longest_str = &str1;
        };

        let divisor = smallest_str.len() as u32;
        let dividend = longest_str.len() as u32;

        let gcd = Solution::find_gcd(divisor, dividend);

        return Solution::check_val(smallest_str, longest_str, gcd as usize);
    }
}

impl LeetCodeTest for Solution {
    fn test_self(self) {
        // assert_eq!(Solution::find_gcd(48, 18), 6);
        // assert_eq!(Solution::find_gcd(54, 24), 6);
        // assert_eq!(Solution::find_gcd(101, 103), 1);
        // assert_eq!(Solution::find_gcd(0, 5), 5);
        // assert_eq!(Solution::find_gcd(5, 0), 5);
        // assert_eq!(Solution::find_gcd(270, 192), 6);
        // assert_eq!(Solution::find_gcd(17, 31), 1);
        // assert_eq!(Solution::find_gcd(100, 10), 10);
        // assert_eq!(Solution::find_gcd(7, 3), 1);

        // let case1 = Solution::gcd_of_strings("ABCABC".to_string(), "ABC".to_string());
        // assert_eq!(case1, "ABC".to_string());

        let case2 = Solution::gcd_of_strings("AA".to_string(), "A".to_string());
        assert_eq!(case2, "A".to_string());
    }
}
