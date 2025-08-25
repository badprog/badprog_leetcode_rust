// https://github.com/badprog/badprog_leetcode_rust

use p_6_zigzag_conversion::Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 3),
            "PAHNAPLSIIGYIR".to_string()
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 4),
            "PINALSIGYAHRPI".to_string()
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::convert("A".to_string(), 1), "A".to_string());
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::convert("AB".to_string(), 1), "AB".to_string());
    }

    #[test]
    fn test_5() {
        assert_eq!(Solution::convert("ABC".to_string(), 2), "ACB".to_string());
    }

    #[test]
    fn test_6() {
        assert_eq!(
            Solution::convert("ABCDE".to_string(), 4),
            "ABCED".to_string()
        );
    }

    #[test]
    fn test_7() {
        assert_eq!(
            Solution::convert("HELLOWORLD".to_string(), 3),
            "HOLELWRDLO".to_string()
        );
        // Pattern: H   O   L
        //          E L W R D
        //          L   O
    }

    #[test]
    fn test_8() {
        assert_eq!(Solution::convert("".to_string(), 1), "".to_string());
    }

    #[test]
    fn test_9() {
        assert_eq!(
            Solution::convert("ABCDEF".to_string(), 5),
            "ABCDFE".to_string()
        );
        // Pattern: A
        //          B
        //          C
        //          D F
        //          E
    }

    #[test]
    fn test_10() {
        assert_eq!(
            Solution::convert("ZIGZAG".to_string(), 3),
            "ZAIZGG".to_string()
        );
        // Pattern: Z   A
        //          I Z G
        //          G
    }
}
