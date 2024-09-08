use crate::common::solution::LeetCodeTest;

pub struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut zp: usize = 0;
        let mut nzp: usize = 0;

        while nzp < nums.len() && zp < nums.len() {
            if nums[zp] == 0 {
                if nums[nzp] != 0 {
                    nums[zp] = nums[nzp];
                    nums[nzp] = 0;
                } else {
                    nzp += 1;
                }
            } else {
                zp += 1;
                nzp += 1;
            }
        }
    }
}

impl LeetCodeTest for Solution {
    fn test_self(self) {
        let mut case1 = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut case1);
        assert_eq!(case1, vec![1, 3, 12, 0, 0]);
    }
}
