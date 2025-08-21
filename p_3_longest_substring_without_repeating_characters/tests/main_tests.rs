// https://github.com/badprog/badprog_leetcode_rust

//
use p_3_longest_substring_without_repeating_characters::Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string() {
        assert_eq!(Solution::length_of_longest_substring(String::from("")), 0);
    }

    #[test]
    fn test_single_char() {
        assert_eq!(Solution::length_of_longest_substring(String::from("a")), 1);
    }

    #[test]
    fn test_all_unique_chars() {
        assert_eq!(
            Solution::length_of_longest_substring(String::from("abcde")),
            5
        );
    }

    #[test]
    fn test_all_same_chars() {
        assert_eq!(
            Solution::length_of_longest_substring(String::from("aaaaa")),
            1
        );
    }

    #[test]
    fn test_repeating_chars() {
        assert_eq!(
            Solution::length_of_longest_substring(String::from("abcabcbb")),
            3
        ); // "abc"
    }

    #[test]
    fn test_partial_repetition() {
        assert_eq!(
            Solution::length_of_longest_substring(String::from("abcdeafghi")),
            9
        ); // "bcdeafghi"
    }

    #[test]
    fn test_two_repetitions() {
        assert_eq!(
            Solution::length_of_longest_substring(String::from("pwwkew")),
            3
        ); // "wke"
    }

    #[test]
    fn test_unicode_chars() {
        assert_eq!(
            Solution::length_of_longest_substring(String::from("世界世界")),
            2
        ); // "世界"
    }

    #[test]
    fn test_mixed_chars() {
        assert_eq!(
            Solution::length_of_longest_substring(String::from("a1b2c3a4")),
            7
        ); // "a1b2c3a"
    }

    #[test]
    fn test_long_repetitive() {
        assert_eq!(
            Solution::length_of_longest_substring(String::from("dvdf")),
            3
        ); // "vdf"
    }
}
