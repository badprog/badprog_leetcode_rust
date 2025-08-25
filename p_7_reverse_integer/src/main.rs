// https://github.com/badprog/badprog_leetcode_rust

use p_7_reverse_integer::Solution;

pub fn main() {
    println!("Hello from Badprog Rust and Leetcode project 6 :D");

    // let x = -100;
    let x = -2147483648;
    let result = Solution::reverse(x);

    println!();
    println!("===========================");
    println!("result: {}", result);
    println!("===========================");
}
