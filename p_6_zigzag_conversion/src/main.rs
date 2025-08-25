// https://github.com/badprog/badprog_leetcode_rust

use p_6_zigzag_conversion::Solution;

pub fn main() {
    println!("Hello from Badprog Rust and Leetcode project 6 :D");

    // let string_to_analyze = "PAYPALISHIRING".to_string();
    let string_to_analyze = "ZIGZAG".to_string();
    let num_rows = 3;
    let result = Solution::convert(string_to_analyze, num_rows);

    println!();
    println!("===========================");
    println!("result: {}", result);
    println!("===========================");
}
