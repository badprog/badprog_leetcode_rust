// https://github.com/badprog/badprog_leetcode_rust

pub struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hash_map: HashMap<i32, i32> = HashMap::new();

        // loop
        for (index, &element) in nums.iter().enumerate() {
            // println!("In the hash_map: {:#?}", hash_map);
            // println!("index = {index:#?}, element = {element:#?}");

            let to_find = target - element;
            // println!("target = {}", target);
            // println!("{} = {} + {}", target, element, "x");
            // println!("So x = {}", to_find);

            if let Some(&get_it) = hash_map.get(&to_find) {
                return vec![get_it, index as i32];
            };

            // println!("");

            hash_map.insert(element, index as i32);
        }

        vec![]
    }
}
