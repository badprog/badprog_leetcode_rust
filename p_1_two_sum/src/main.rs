//
use p_1_two_sum::Solution;

// ============================================================================
// main
fn main() {
    println!("Hello, world!");

    let nums = vec![2, 7, 11, 15];
    let target = 26;

    let result = Solution::two_sum(nums, target);
    println!("result = {:#?}", result);
}
