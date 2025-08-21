// https://github.com/badprog/badprog_leetcode_rust

use p_3_longest_substring_without_repeating_characters::Solution;

pub fn main() {
    println!("Hello from Badprog Rust and Leetcode project 3 :D");

    // let s = "".to_string(); // 0
    // let s = "a".to_string(); // 1
    // let s = "abcabcbb".to_string(); // 3
    // let s = "au".to_string(); // 2
    // let s = "aab".to_string();
    // let s = "dvdf".to_string(); // 3
    // let s = "abcdaefgcehi".to_string(); // 7
    let s = "éllo Wörld ラ!".to_string();
    // let s = "HeyHey!".to_string();
    // let s = "Hello World!Wat".to_string();
    // let s = "Hell".to_string();
    // let s = "dvdf".to_string();
    // let s = "hijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789hijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789hijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789hijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789hijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789hijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".to_string();
    let result = Solution::length_of_longest_substring(s);

    println!("");
    println!("result = {:#?}", result);
}
