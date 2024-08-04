use crate::common::solution::LeetCodeTest;

pub struct Solution;

impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut i: usize = 0;
        while i < nums.len() {
            let num1 = nums.get(i).unwrap();
            for j in (i + 1..nums.len()).collect::<Vec<usize>>() {
                let num2 = nums.get(j).unwrap();
                if *num1 < *num2 {
                    for k in (j + 1..nums.len()).collect::<Vec<usize>>() {
                        let num3 = nums.get(k).unwrap();
                        if *num2 < *num3 {
                            return true;
                        }
                    }
                }
            }
            i += 1;
        }

        return false;
    }
}

impl LeetCodeTest for Solution {
    fn test_self(self) {
        let case1 = Solution::increasing_triplet(vec![5, 4, 3, 2, 1]);
        assert!(case1 == false);

        let case2 = Solution::increasing_triplet(vec![2, 1, 5, 0, 4, 6]);
        assert!(case2 == true);

        let case3 = Solution::increasing_triplet(vec![0, 4, 2, 1, 0, -1, -3]);
        assert!(case3 == false);
    }
}
