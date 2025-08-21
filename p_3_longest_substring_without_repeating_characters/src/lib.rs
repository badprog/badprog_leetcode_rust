// https://github.com/badprog/badprog_leetcode_rust

pub struct Solution;

// =========================================================================
//
impl Solution {
    // // =========================================================================
    // //
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut hash_map = std::collections::HashMap::new();
        let mut max_length = 0;
        let mut start = 0;

        for (index, element) in s.chars().enumerate() {
            if let Some(&position) = hash_map.get(&element) {
                if position >= start {
                    start = position + 1;
                }
            }
            hash_map.insert(element, index);
            max_length = max_length.max((index - start + 1) as i32);
        }

        max_length
    }
}
