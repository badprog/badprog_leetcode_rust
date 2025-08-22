// https://github.com/badprog/badprog_leetcode_rust

use std::ops::Div;

pub struct Solution;

// =========================================================================
//
impl Solution {
    // =========================================================================
    //
    fn check_if_empty(nums1: &[i32], nums2: &[i32]) -> Option<f64> {
        // =========================================================================
        //
        fn median_of_single_array(arr: &[i32]) -> f64 {
            let mid = arr.len() / 2;
            if arr.len() % 2 == 0 {
                // size is an even number, so we get the 2 middle values and divide them by 2
                (arr[mid - 1] as f64 + arr[mid] as f64) / 2.0
            } else {
                // otherwise we have an odd number, so we get the middle value
                arr[mid] as f64
            }
        }

        // if nums1.is_empty() && nums2.is_empty() { --> This case isn't allowed by constraints
        //     return None;
        // }
        if nums1.is_empty() {
            return Some(median_of_single_array(nums2));
        }
        if nums2.is_empty() {
            return Some(median_of_single_array(nums1));
        }
        None
    }

    // =========================================================================
    //
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        //
        if let Some(median) = Self::check_if_empty(&nums1, &nums2) {
            return median;
        }

        // We ensure that nums1 is lesser than nums2 in order to optimize
        let (nums1, nums2) = if nums1.len() > nums2.len() {
            (nums2, nums1)
        } else {
            (nums1, nums2)
        };

        let n1 = nums1.len();
        let n2 = nums2.len();
        let mut low = 0;
        let mut high = n1;

        while low <= high {
            let part_x = (low + high) / 2; // Example nums1[1,4,6] -> (0 + 3) /2
            let part_y = (n1 + n2 + 1).div(2) - part_x; // Example nums2[2,3,4,7,8,9] -> (3 + 6 + 1) / 2

            // =========================================================================
            // MIN and MAX are used in order to prevent crash when accessing a data out of bound.
            // Example: arr[-1] or arr[-2], so we get the most low value in order to prevent this case (even if it's very low).
            fn get_boundary(arr: &[i32], index: usize, is_left: bool, max_len: usize) -> i32 {
                if is_left {
                    if index == 0 { i32::MIN } else { arr[index - 1] }
                } else if index == max_len {
                    i32::MAX
                } else {
                    arr[index]
                }
            }

            //
            let left_x = get_boundary(&nums1, part_x, true, n1);
            let right_x = get_boundary(&nums1, part_x, false, n1);
            let left_y = get_boundary(&nums2, part_y, true, n2);
            let right_y = get_boundary(&nums2, part_y, false, n2);

            //
            if left_x <= right_y && left_y <= right_x {
                // Correct partition
                return if (n1 + n2) % 2 == 0 {
                    (left_x.max(left_y) as f64 + right_x.min(right_y) as f64) / 2.0
                } else {
                    left_x.max(left_y) as f64
                };
            } else if left_x > right_y {
                high = part_x - 1;
            } else {
                low = part_x + 1;
            }
        }

        0.0 // Not supposed to happen
    }

    //// =========================================================================
    ////
    pub fn find_median_sorted_arrays_2(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let result;

        let mut array_sorted = nums1;
        array_sorted.extend(nums2);
        array_sorted.sort();
        println!("array_sorted = {:?}", array_sorted);

        let array_len = array_sorted.len();
        println!("size_array = {:?}", array_len);

        let modulo = array_len % 2;
        println!("modulo = {:?}", modulo);
        // let mut middle = array_len / 2;
        let index = array_len / 2;
        if modulo == 0 {
            if array_len == 2 {
                let number_1 = *array_sorted.first().unwrap() as f64;
                let number_2 = *array_sorted.get(1).unwrap() as f64;

                result = (number_1 + number_2) / 2.0;

                return result;
            }
            let middle_even_1 = array_sorted.get(index - 1).unwrap();
            let middle_even_2 = array_sorted.get(index).unwrap();
            println!("Array is even, middle_even_1 = {}", middle_even_1);
            println!("Array is even, middle_even_2 = {}", middle_even_2);
            result = (*middle_even_1 as f64 + *middle_even_2 as f64) / 2.0;
        } else {
            let middle_odd = array_sorted.get(index).unwrap();
            println!("Array is odd, middle_odd = {}", middle_odd);
            result = *middle_odd as f64;
        }

        let number_1 = array_sorted.get(array_len);
        println!("number_1 = {:?}", number_1);

        result
    }
}
