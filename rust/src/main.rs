mod common;
mod graphs_dfs;
mod linked_list;
mod lists;
mod prefix_sum;
mod sliding_window;
mod strings;
mod two_pointer;

use common::solution::{self, LeetCodeTest};

fn main() {
    // let solution = lists::merge_alternate::Solution {};
    // let solution = lists::container_with_most_water::Solution {};
    // let solution = lists::max_k_sum_pairs::Solution {};
    // let solution = lists::max_vowels_in_substring::Solution {};
    // let solution = linked_list::min_max_node::Solution {};
    // let solution = lists::string_compression::Solution {};
    // let solution = lists::increasing_triplet_sequence::Solution {};
    // let solution = strings::gcd_of_strings_1::Solution {};
    // let solution = two_pointer::move_zeroes::Solution {};
    // let solution = sliding_window::max_consecutive_ones::Solution {};
    // let solution = prefix_sum::highest_altitude::Solution {};
    let solution = graphs_dfs::keys_and_rooms::Solution {};
    solution.test_self();
}
