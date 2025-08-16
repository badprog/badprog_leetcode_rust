// https://github.com/badprog/badprog_leetcode_rust

use p_2_add_two_numbers::{ListNode, Solution};

pub fn main() {
    println!("Hello from Badprog Rust and Leetcode project 2 :D");
    // println!("");

    // let vec1  = vec![2, 4, 3];
    // let vec2  = vec![5, 6, 4];
    // let vec1  = vec![0];
    // let vec2  = vec![0];
    // let vec1  = vec![9,9,9,9,9,9,9];
    // let vec2  = vec![9,9,9,9];
    let vec1 = [2, 4, 9];
    let vec2 = None; //vec![5, 6, 4, 9];
    // let vec1  = vec![0];
    // let vec2  = vec![1];
    // let vec1 = vec![1, 2, 3];
    // let vec2 = vec![4, 5, 6, 7];

    let mut l1 = None;
    let mut l2 = None;
    let mut current1 = &mut l1;
    let mut current2 = &mut l2;

    for &val in vec1.iter() {
        *current1 = Some(Box::new(ListNode { val, next: None }));
        current1 = &mut current1.as_mut().unwrap().next;
    }

    for &val in vec2.iter() {
        *current2 = Some(Box::new(ListNode { val, next: None }));
        current2 = &mut current2.as_mut().unwrap().next;
    }

    let solution = Solution::add_two_numbers(l1, l2);

    // println!("");
    println!("Solution = {:?}", solution);

    let mut result = Vec::new();
    let mut temp = &solution;
    while let Some(node) = temp {
        result.push(node.val);
        temp = &node.next;
    }
    println!("Result as vector: {:?}", result);
}
