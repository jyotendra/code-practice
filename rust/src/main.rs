use common::solution::{self, LeetCodeTest};

mod common;
mod lists;

fn main() {
    // let solution = lists::merge_alternate::Solution {};
    // let solution = lists::container_with_most_water::Solution {};
    // let solution = lists::max_k_sum_pairs::Solution {};
    let solution = lists::max_vowels_in_substring::Solution {};
    solution.test_self();
}
