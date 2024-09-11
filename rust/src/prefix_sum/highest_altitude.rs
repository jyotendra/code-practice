// There is a biker going on a road trip. The road trip consists of n + 1 points at different altitudes. The biker starts his trip on point 0 with altitude equal 0.
//
// You are given an integer array gain of length n where gain[i] is the net gain in altitude between points i​​​​​​ and i + 1 for all (0 <= i < n). Return the highest altitude of a point.
//
// Example 1:
//
// Input: gain = [-5,1,5,0,-7]
// Output: 1
// Explanation: The altitudes are [0,-5,-4,1,1,-6]. The highest is 1.
// Example 2:
//
// Input: gain = [-4,-3,-2,-1,4,3,2]
// Output: 0
// Explanation: The altitudes are [0,-4,-7,-9,-10,-6,-3,-1]. The highest is 0.
//
// Constraints:
//
// n == gain.length
// 1 <= n <= 100
// -100 <= gain[i] <= 100

use crate::common::solution::LeetCodeTest;
use std::cmp::max;

pub struct Solution;

impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut max_gain = 0;
        gain.iter().fold(0, |acc, e| {
            let new_val = acc + *e;
            max_gain = max(max_gain, new_val);
            new_val
        });
        max_gain
    }
}

impl LeetCodeTest for Solution {
    fn test_self(self) {
        let case1 = Solution::largest_altitude(vec![-5, 1, 5, 0, -7]);
        assert!(case1 == 1);

        let case2 = Solution::largest_altitude(vec![-4, -3, -2, -1, 4, 3, 2]);
        assert!(case2 == 0);
    }
}
