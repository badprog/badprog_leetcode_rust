// ============================================================================
// use
use p_1_two_sum::Solution;

// ============================================================================
// test
#[test]
fn test_two_sum_basic() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = Solution::two_sum(nums, target);
    assert_eq!(result, vec![0, 1]);
}

#[test]
fn test_two_sum_no_solution() {
    let nums = vec![1, 2, 3, 4];
    let target = 10;
    let result = Solution::two_sum(nums, target);
    assert_eq!(result, vec![]);
}

#[test]
fn test_two_sum_negative_numbers() {
    let nums = vec![-3, 4, 3, 90];
    let target = 0;
    let result = Solution::two_sum(nums, target);
    assert_eq!(result, vec![0, 2]);
}

#[test]
fn test_two_sum_duplicate_numbers() {
    let nums = vec![3, 3];
    let target = 6;
    let result = Solution::two_sum(nums, target);
    assert_eq!(result, vec![0, 1]);
}

#[test]
fn test_two_sum_single_solution() {
    let nums = vec![1, 5, 5, 10];
    let target = 10;
    let result = Solution::two_sum(nums, target);
    assert_eq!(result, vec![1, 2]);
}
