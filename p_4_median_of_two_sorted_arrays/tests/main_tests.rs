// https://github.com/badprog/badprog_leetcode_rust

use p_4_median_of_two_sorted_arrays::Solution;

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_nums1_empty() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![], vec![1]), 1.0);
    }

    #[test]
    fn test_nums2_empty() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![1], vec![]), 1.0);
    }

    #[test]
    fn test_single_element_each() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![1], vec![2]), 1.5);
    }

    #[test]
    fn test_odd_total_length() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2, 4, 5]),
            3.0
        );
    }

    #[test]
    fn test_even_total_length() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.5
        );
    }

    #[test]
    fn test_max_size() {
        let nums1 = vec![1; 1000];
        let nums2 = vec![2; 1000];
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 1.5);
    }

    #[test]
    fn test_min_values() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![-1000000], vec![-1000000]),
            -1000000.0
        );
    }

    #[test]
    fn test_max_values() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1000000], vec![1000000]),
            1000000.0
        );
    }

    #[test]
    fn test_mixed_values() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![-1000000, 0], vec![1000000]),
            0.0
        );
    }

    #[test]
    fn test_large_uneven_arrays() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4, 5, 6, 7]),
            4.0
        );
    }
}
