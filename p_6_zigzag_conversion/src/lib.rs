// https://github.com/badprog/badprog_leetcode_rust

pub struct Solution;

// =========================================================================
//
impl Solution {
    // =========================================================================
    //
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 || num_rows >= s.len() as i32 {
            return s;
        }

        // We need 1 vector with 'num_rows' elements (here elements are Strings)
        let mut rows = vec![String::new(); num_rows as usize];
        let mut cur_row = 0;
        let mut step = 1;

        for element in s.chars() {
            rows[cur_row as usize].push(element);
            // println!("rows: {:?}", rows);
            if cur_row == 0 {
                step = 1;
            } else if cur_row == num_rows - 1 {
                step = -1;
            }
            cur_row += step;
        }

        rows.concat()
    }
}
