// https://github.com/badprog/badprog_leetcode_rust

pub struct Solution;

// =========================================================================
//
impl Solution {
    // =========================================================================
    //
    pub fn reverse(x: i32) -> i32 {
        //
        let sign = if x < 0 { -1 } else { 1 };
        let mut str_from_x = x.to_string();

        //
        if x < 0 {
            str_from_x = str_from_x[1..].to_string();
        }

        let reversed = str_from_x.chars().rev().collect::<String>();

        //
        match reversed.parse::<i32>() {
            Ok(num) => num * sign,
            Err(_) => 0, // overflow or error
        }
    }

    // =========================================================================
    //
    pub fn reverse2(x: i32) -> i32 {
        let mut x = x;
        let mut result = 0;

        while x != 0 {
            // Get the last digit
            let digit = x % 10;
            // Remove the last digit and update x
            x /= 10;

            // Check overflow
            // if result > (i32::MAX / 10) || result < (i32::MIN / 10) {
            if !(i32::MIN / 10..=i32::MAX / 10).contains(&result) {
                return 0;
            }

            // Create the result by adding the number (multiplied by 10 for each step)
            result = result * 10 + digit;
        }

        result
    }
}
