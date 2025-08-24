// https://github.com/badprog/badprog_leetcode_rust

pub struct Solution;

// =========================================================================
//
impl Solution {
    // =========================================================================
    //
    pub fn longest_palindrome(s: String) -> String {
        if s.is_empty() {
            return String::new();
        }

        // Create a new string with #
        let str_to_analyze: String = format!(
            "#{}#", // add 1 '#' at the beginning and the end -> {} is the main string
            s.chars()
                .map(|c| c.to_string()) // each char becomes a string
                .collect::<Vec<String>>() // add them all in a Vec of String
                .join("#")  // create a unique string with all strings from the Vec
        );

        // println!("str_to_analyze = {}", str_to_analyze);

        // std::process::exit(-1337);

        let str_len = str_to_analyze.len();
        let mut palindrom = vec![0; str_len]; // Length of a new palindrom
        let (mut bound_center, mut bound_right) = (0, 0); // Center and right bound of the current palindrom

        // Palindrom length
        for i in 1..str_len - 1 {
            if i < bound_right {
                // println!("i: {}, bound_right: {}", i, bound_right);
                palindrom[i] = palindrom[2 * bound_center - i].min(bound_right - i);
                // println!("palindrom[i]: {}", palindrom[i]);
            }

            //
            while i + palindrom[i] + 1 < str_len
                && i >= palindrom[i] + 1
                && str_to_analyze.chars().nth(i + palindrom[i] + 1)
                    == str_to_analyze.chars().nth(i - palindrom[i] - 1)
            {
                // Each center will have the number of their max side as a palindrom;
                // 0, 1, 2, 1, 3, 1, 0
                // -> index with 3 means 2 letters on the right and 2 on the left (1 + 2 * 2 so 5 letters)
                palindrom[i] += 1;
            }

            //
            if i + palindrom[i] > bound_right {
                bound_center = i;
                bound_right = i + palindrom[i];
            }
        }

        // Find the max length palindrom
        let max_len = palindrom.iter().max().unwrap_or(&0);
        let center = palindrom.iter().position(|&x| x == *max_len).unwrap_or(0);
        let start = (center - max_len) / 2;
        let end = start + max_len;

        s[start..end].to_string()
    }

    // =========================================================================
    //
    pub fn longest_palindrome2(s: String) -> String {
        let mut str_tmp = "".to_string();
        let mut index_tmp = 0;
        let mut max_length = 0;
        let string_default = &s;
        let mut string_to_return = "".to_string();

        // println!("String default = {}", &string_default);

        let mut size_string = s.len();
        let mut counter = 0;
        let mut idx = 0;
        let mut max_of_max = 0;

        //
        while counter < size_string {
            // println!(
            //     "==================== while n{} =====================",
            //     counter
            // );
            str_tmp.clear();
            index_tmp = 0;

            //
            for (index, element) in string_default.chars().skip(counter).enumerate() {
                //
                // println!("");
                // println!("index: {}, element: {}", index, element);

                // tmp
                str_tmp.insert(index_tmp, element);
                // println!("str_tmp = {}", str_tmp);
                let str_rev: String = str_tmp.chars().rev().collect();
                // println!("str_rev = {}", str_rev);

                if str_tmp == str_rev {
                    // println!(
                    // "We got a match: str_tmp = {}, str_rev = {}",
                    //     str_tmp,
                    //     str_rev
                    // );

                    if max_length < str_tmp.len() {
                        max_length = str_tmp.len();
                        // println!("- - > max_length = {}", max_length);
                    }
                    if max_of_max < max_length {
                        string_to_return = str_tmp.clone();
                        // println!("- - - - - - > max_of_max = {}", max_of_max);
                        max_of_max = max_length;
                    }
                }

                index_tmp += 1;

                //
            } // END - for

            counter += 1;
        } // END - while

        //
        string_to_return
    }
}
