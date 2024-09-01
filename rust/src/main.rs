mod common;
mod linked_list;
mod lists;
mod strings;

use common::solution::LeetCodeTest;

fn main() {
    // let solution = lists::merge_alternate::Solution {};
    // let solution = lists::container_with_most_water::Solution {};
    // let solution = lists::max_k_sum_pairs::Solution {};
    // let solution = lists::max_vowels_in_substring::Solution {};
    // let solution = linked_list::min_max_node::Solution {};
    // let solution = lists::string_compression::Solution {};
    // let solution = lists::increasing_triplet_sequence::Solution {};
    let solution = strings::gcd_of_strings_1::Solution {};
    solution.test_self();
}
