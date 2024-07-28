// https://leetcode.com/problems/find-the-minimum-and-maximum-number-of-nodes-between-critical-points/

use crate::common::solution::LeetCodeTest;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::vec;

pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32, reference: Option<Box<ListNode>>) -> Self {
        ListNode {
            next: reference,
            val,
        }
    }
}

struct CriticalPoint {
    index: usize,
    val: i32,
}

pub struct Solution;

impl Solution {
    pub fn nodes_between_critical_points(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut current_head = head;
        let mut prev_val: Option<i32> = None;
        let mut min_max_vec: Vec<CriticalPoint> = vec![];
        let mut i = 0;
        while let Some(node) = current_head {
            // Assign the value of the current node to prev_val before moving to the next.
            let current_val = Some(node.val);

            // Directly map over node.next to extract the value if it exists, simplifying the assignment to next_val.
            let next_val = node.next.as_ref().map(|next_node| next_node.val);

            if prev_val.is_some() && next_val.is_some() {
                if prev_val.unwrap() < current_val.unwrap()
                    && next_val.unwrap() < current_val.unwrap()
                {
                    // condition of maxima
                    min_max_vec.push(CriticalPoint {
                        index: i,
                        val: current_val.unwrap(),
                    });
                }

                if prev_val.unwrap() > current_val.unwrap()
                    && next_val.unwrap() > current_val.unwrap()
                {
                    // condition of minima
                    min_max_vec.push(CriticalPoint {
                        index: i,
                        val: current_val.unwrap(),
                    });
                }
            }

            // Move to the next node in the list.
            prev_val = current_val;
            current_head = node.next;
            i += 1;
        }

        if min_max_vec.len() < 2 {
            return vec![-1, -1];
        }

        let mut min_dist: Option<usize> = None;
        let max_dist = min_max_vec
            .get((min_max_vec.len() - 1) as usize)
            .unwrap()
            .index
            - min_max_vec.get(0).unwrap().index;

        for window in min_max_vec.windows(2) {
            if let [cp_0, cp_1] = window {
                let current_dist = cp_1.index - cp_0.index;
                min_dist = Some(min(current_dist, min_dist.unwrap_or(current_dist)));
            }
        }

        vec![min_dist.unwrap() as i32, max_dist as i32]
    }
}

impl LeetCodeTest for Solution {
    fn test_self(self) {
        let case1_ll = Some(Box::new(ListNode::new(
            3,
            Some(Box::new(ListNode::new(1, None))),
        )));
        let sol1 = Solution::nodes_between_critical_points(case1_ll);
        println!("{:?}", sol1);
        assert_eq!(sol1, vec![-1, -1]);

        // ========
        let case2_ll = Some(Box::new(ListNode::new(
            5,
            Some(Box::new(ListNode::new(
                3,
                Some(Box::new(ListNode::new(
                    1,
                    Some(Box::new(ListNode::new(
                        2,
                        Some(Box::new(ListNode::new(
                            5,
                            Some(Box::new(ListNode::new(
                                1,
                                Some(Box::new(ListNode::new(2, None))),
                            ))),
                        ))),
                    ))),
                ))),
            ))),
        )));

        let sol2 = Solution::nodes_between_critical_points(case2_ll);
        println!("{:?}", sol2);
        assert_eq!(sol2, vec![1, 3]);
    }
}
