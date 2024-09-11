// There are n rooms labeled from 0 to n - 1 and all the rooms are locked except for room 0. Your goal is to visit all the rooms. However, you cannot enter a locked room without having its key.
//
// When you visit a room, you may find a set of distinct keys in it. Each key has a number on it, denoting which room it unlocks, and you can take all of them with you to unlock the other rooms.
//
// Given an array rooms where rooms[i] is the set of keys that you can obtain if you visited room i, return true if you can visit all the rooms, or false otherwise.
//
//
//
// Example 1:
//
// Input: rooms = [[1],[2],[3],[]]
// Output: true
// Explanation:
// We visit room 0 and pick up key 1.
// We then visit room 1 and pick up key 2.
// We then visit room 2 and pick up key 3.
// We then visit room 3.
// Since we were able to visit every room, we return true.
// Example 2:
//
// Input: rooms = [[1,3],[3,0,1],[2],[0]]
// Output: false
// Explanation: We can not enter room number 2 since the only key that unlocks it is in that room.

use crate::common::solution::LeetCodeTest;

pub struct Solution;

impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let mut visited_rooms: Vec<Option<bool>> = vec![None; rooms.len()];
        let mut collected_keys: Vec<i32> = vec![];
        collected_keys.extend(rooms[0].iter());
        visited_rooms[0] = Some(true);

        while collected_keys.len() > 0 {
            let key = collected_keys.pop().unwrap() as usize;
            visited_rooms[key] = Some(true);
            for k in rooms[key].iter() {
                if visited_rooms[*k as usize] == None {
                    collected_keys.push(*k);
                }
            }
        }

        return visited_rooms
            .iter()
            .fold(true, |acc, r| acc && (*r).unwrap_or(false));
    }
}

impl LeetCodeTest for Solution {
    fn test_self(self) {
        let case1 = Solution::can_visit_all_rooms(vec![vec![1], vec![2], vec![3], vec![]]);
        assert_eq!(case1, true);
    }
}
