use crate::common::solution::LeetCodeTest;
use std::cmp::{max, min};

pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = height.len() - 1;
        let mut max_area = 0;
        while i <= j {
            let left_height = height[i];
            let right_height = height[j];
            let min_height = min(left_height, right_height);
            let width = (j - i) as i32;
            let current_area = min_height * width;

            max_area = max(max_area, current_area);

            if left_height <= right_height {
                i += 1;
            } else if left_height > right_height {
                j -= 1;
            }
        }

        return max_area;
    }
}

impl LeetCodeTest for Solution {
    fn test_self(self) {
        let case1 = Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]);
        println!("{}", case1);
        assert!(case1 == 49);
    }
}
