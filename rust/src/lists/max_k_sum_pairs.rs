use crate::common::solution::LeetCodeTest;
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        let mut dict_map: HashMap<i32, i32> = nums.iter().fold(HashMap::new(), |mut hash, val| {
            *hash.entry(*val).or_insert(0) += 1;
            hash
        });

        let mut num_operations = 0;

        while nums.len() > 0 {
            // pop last value
            let last_val = nums.pop();
            if last_val != None {
                let remananat = k - last_val.unwrap();
                if dict_map.contains_key(&remananat) {
                    let last_val_count = dict_map.get(&last_val.unwrap()).unwrap();
                    if *last_val_count > 0 {
                        let existing_remanant_entry_val = dict_map.get_mut(&remananat).unwrap();

                        let remanant_count = if remananat == last_val.unwrap() { 1 } else { 0 };
                        if *existing_remanant_entry_val > remanant_count {
                            num_operations += 1;
                            *existing_remanant_entry_val -= 1;
                            dict_map.entry(last_val.unwrap()).and_modify(|v| *v -= 1);
                        }
                    }
                }
            }
        }

        num_operations
    }
}

impl LeetCodeTest for Solution {
    fn test_self(self) {
        let case1 = Solution::max_operations(vec![2, 2, 2, 3, 1, 1, 4, 1], 4);
        println!("Result ----> {}", case1);
        assert!(case1 == 2);
    }
}
