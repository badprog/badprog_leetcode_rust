// https://github.com/badprog/badprog_leetcode_rust

use p_5_longest_palindromic_substring::Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string() {
        assert_eq!(Solution::longest_palindrome(String::from("")), "");
    }

    #[test]
    fn test_single_character() {
        assert_eq!(Solution::longest_palindrome(String::from("a")), "a");
    }

    #[test]
    fn test_two_same_characters() {
        assert_eq!(Solution::longest_palindrome(String::from("aa")), "aa");
    }

    #[test]
    fn test_two_different_characters() {
        assert_eq!(Solution::longest_palindrome(String::from("ab")), "a"); // ou "b"
    }

    #[test]
    fn test_palindrome_odd_length() {
        assert_eq!(Solution::longest_palindrome(String::from("babad")), "bab"); // ou "aba"
    }

    #[test]
    fn test_palindrome_even_length() {
        assert_eq!(Solution::longest_palindrome(String::from("cbbd")), "bb");
    }

    #[test]
    fn test_full_palindrome() {
        assert_eq!(
            Solution::longest_palindrome(String::from("racecar")),
            "racecar"
        );
    }

    #[test]
    fn test_no_palindrome_longer_than_one() {
        assert_eq!(Solution::longest_palindrome(String::from("abcdef")), "a"); // ou tout autre caractère unique
    }

    #[test]
    fn test_repeated_characters() {
        assert_eq!(Solution::longest_palindrome(String::from("aaaa")), "aaaa");
    }

    #[test]
    fn test_complex_string() {
        assert_eq!(
            Solution::longest_palindrome(String::from("abracadabra")),
            "aca"
        );
    }
}
