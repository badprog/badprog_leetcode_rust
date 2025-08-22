// https://github.com/badprog/badprog_leetcode_rust

use p_4_median_of_two_sorted_arrays::Solution;

pub fn main() {
    println!("Hello from Badprog Rust and Leetcode project 4 :D");

    // let nums1 = vec![1, 2, 3];
    // let nums2 = vec![4, 5, 6, 7];
    // let nums1 = vec![1];
    // let nums2 = vec![0];
    let nums1 = vec![1, 2];
    let nums2 = vec![3, 4];
    let result = Solution::find_median_sorted_arrays(nums1, nums2);

    println!("result = {}", result);
}
