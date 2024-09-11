use crate::common::solution::LeetCodeTest;
/**
* Given a binary array nums and an integer k,
* return the maximum number of consecutive 1's
* in the array if you can flip at most k 0's.

   Example 1:

   Input: nums = [1,1,1,0,0,0,1,1,1,1,0], k = 2
   Output: 6
   Explanation: [1,1,1,0,0,1,1,1,1,1,1]
   Bolded numbers were flipped from 0 to 1. The longest subarray is underlined.
   Example 2:

   Input: nums = [0,0,1,1,0,0,1,1,1,0,1,1,0,0,0,1,1,1,1], k = 3
   Output: 10
   Explanation: [0,0,1,1,1,1,1,1,1,1,1,1,0,0,0,1,1,1,1]
   Bolded numbers were flipped from 0 to 1. The longest subarray is underlined.``
*/

pub struct Solution;

impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let mut l_i = 0;
        let mut r_i = 0;
        let mut current_zeroes = 0;

        while r_i < nums.len() {
            if nums[r_i] == 0 {
                current_zeroes += 1;
            }
            if current_zeroes > k {
                if nums[l_i] == 0 {
                    current_zeroes -= 1;
                }
                l_i += 1;
            }
            r_i += 1;
        }
        return (r_i - l_i) as i32;
    }
}

impl LeetCodeTest for Solution {
    fn test_self(self) {
        let case1 = Solution::longest_ones(vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0], 2);
        assert_eq!(case1, 6);

        let case2 = Solution::longest_ones(
            vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1],
            3,
        );
        assert_eq!(case2, 10);

        let case3 = Solution::longest_ones(vec![0, 0, 0, 1], 3);
        assert_eq!(case3, 4);
    }
}
