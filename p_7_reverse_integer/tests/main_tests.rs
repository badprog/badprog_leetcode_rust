// https://github.com/badprog/badprog_leetcode_rust

use p_7_reverse_integer::Solution;

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_positive_number() {
        assert_eq!(Solution::reverse(123), 321);
    }

    #[test]
    fn test_negative_number() {
        assert_eq!(Solution::reverse(-123), -321);
    }

    #[test]
    fn test_zero() {
        assert_eq!(Solution::reverse(0), 0);
    }

    #[test]
    fn test_trailing_zeros() {
        assert_eq!(Solution::reverse(120), 21);
    }

    #[test]
    fn test_single_digit() {
        assert_eq!(Solution::reverse(5), 5);
    }

    #[test]
    fn test_overflow_positive() {
        assert_eq!(Solution::reverse(1534236469), 0);
    }

    #[test]
    fn test_overflow_negative() {
        assert_eq!(Solution::reverse(-1534236469), 0);
    }

    #[test]
    fn test_max_i32() {
        assert_eq!(Solution::reverse(2147483647), 0);
    }

    #[test]
    fn test_min_i32() {
        assert_eq!(Solution::reverse(-2147483648), 0);
    }

    #[test]
    fn test_multiple_zeros() {
        assert_eq!(Solution::reverse(1000), 1);
    }
}
