// https://github.com/badprog/badprog_leetcode_rust

// Importing types from the p_2_add_two_numbers crate.
use p_2_add_two_numbers::{ListNode, Solution};

#[cfg(test)]
mod tests {
    use super::*; // adding the types fron p_2 in this sub-module tests.

    // Utility function to create a linked list from a vector
    fn create_list(values: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut current = &mut head;
        for &val in values.iter().rev() {
            *current = Some(Box::new(ListNode { val, next: None }));
            current = &mut current.as_mut().unwrap().next;
        }
        head
    }

    // Utility function to convert a linked list to a vector
    fn list_to_vec(list: Option<Box<ListNode>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut current = list;
        while let Some(node) = current {
            result.push(node.val);
            current = node.next;
        }
        result
    }

    #[test]
    fn test_empty_lists() {
        let l1 = None;
        let l2 = None;
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(list_to_vec(result), vec![]);
    }

    #[test]
    fn test_one_empty_list() {
        let l1 = create_list(vec![2, 4, 3]);
        let l2 = None;
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(list_to_vec(result), vec![3, 4, 2]);
    }

    #[test]
    fn test_equal_length_no_carry() {
        let l1 = create_list(vec![2, 4, 3]);
        let l2 = create_list(vec![5, 6, 4]);
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(list_to_vec(result), vec![7, 0, 8]); // 342 + 465 = 807
    }

    #[test]
    fn test_different_length_with_carry() {
        let l1 = create_list(vec![9, 9, 9]);
        let l2 = create_list(vec![1]);
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(list_to_vec(result), vec![0, 0, 0, 1]); // 999 + 1 = 1000
    }

    #[test]
    fn test_carry_propagation() {
        let l1 = create_list(vec![9, 9, 9, 9]);
        let l2 = create_list(vec![9, 9, 9, 9]);
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(list_to_vec(result), vec![8, 9, 9, 9, 1]); // 9999 + 9999 = 19998
    }

    #[test]
    fn test_single_digit() {
        let l1 = create_list(vec![5]);
        let l2 = create_list(vec![5]);
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(list_to_vec(result), vec![0, 1]); // 5 + 5 = 10
    }
}
