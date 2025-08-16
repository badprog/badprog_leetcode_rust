// https://github.com/badprog/badprog_leetcode_rust

pub struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// =========================================================================
//
impl Solution {
    // =========================================================================
    //
    // fn display(option_to_display: &Option<Box<ListNode>>) {
    //
    // println!("========== fn option_to_display");

    //
    // match option_to_display.as_ref() {
    // Some(element) => {
    // println!("a. element.val = {}", element.val);
    // }
    // None => {
    // println!("a. None");
    // }
    // }

    //
    // if let Some(element) = &option_to_display {
    // println!("b. element.val = {}", element.val);
    // } else {
    // println!("b. None");
    // }
    // }

    // =========================================================================
    //
    pub fn add(option_to_modify: &mut Option<Box<ListNode>>, number_to_add: &i32) {
        //
        // println!("========== fn option_to_modify");

        // Option empty
        if option_to_modify.is_none() {
            // println!("option_to_modify.is_none(), need to create one some(box::new(nodelist::new");
            return;
        }
        let mut current_option = option_to_modify;

        // Option has at least 1 element
        while let Some(element) = current_option.as_mut() {
            if element.next.is_some() {
                current_option = &mut element.next;
            } else {
                element.next = Some(Box::new(ListNode {
                    val: *number_to_add,
                    next: (None),
                }));

                // println!("c. current element.val = {}", element.val);
                // println!("c. number_to_add in the next= {}", number_to_add);
                // println!("c. Verification:");

                // let number_added = element
                // .next
                // .as_ref()
                // .and_then(|next_node| Some(next_node.val));

                // println!("Next val just added = {:?}", number_added);
                break;
            }
        }
        // match option_to_modify.as_mut() {
        //     Some(element) => {
        //         element.val = *number_to_add;
        //         println!("a. element.val = {}", element.val);
        //     }
        //     None => {
        //         println!("a. None");
        //     }
        // }
    }

    // =========================================================================
    //
    fn retrieve_first_val_element(option_to_analyze: &mut Option<Box<ListNode>>) -> i32 {
        //
        // println!("========== fn retrieve_first_val_element");

        //
        let mut result = 0;
        // println!("option_to_analyze = {:?}", option_to_analyze);

        //
        if let Some(element) = option_to_analyze.as_mut() {
            //
            // println!("element.val = {:?}", element.val);

            result = element.val;

            *option_to_analyze = element.next.take();
        }

        //
        result
    }

    // =========================================================================
    //
    fn remove_first_element(option_to_manage: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        option_to_manage.and_then(|node| node.next)
    }

    // =========================================================================
    //
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        //
        // println!("========== fn add_two_numbers");

        if l1.is_none() && l2.is_none() {
            return None;
        }

        if l1.is_none() {
            return l2;
        }

        if l2.is_none() {
            return l1;
        }

        //
        let mut option_1 = Some(Box::new(ListNode::new(0)));

        //
        // Self::display(&option_1);

        // Self::add(&mut option_1, &18);
        // Self::add(&mut option_1, &19);
        // Self::add(&mut option_1, &20);
        // Self::add(&mut option_1, &21);

        let mut current_1 = l1;
        let mut current_2 = l2;

        let mut result_1;
        let mut result_2;
        let mut number_total;
        let mut carry = 0;

        while current_1.is_some() || current_2.is_some() || carry != 0 {
            number_total = 0;
            number_total += carry;
            // carry = 0;

            // while (current_1.is_some()) {
            result_1 = Self::retrieve_first_val_element(&mut current_1);
            // println!("result_1 = {}", result_1);
            // }

            // while (current_2.is_some()) {
            result_2 = Self::retrieve_first_val_element(&mut current_2);
            // println!("result_2 = {}", result_2);
            // }

            number_total += result_1 + result_2;
            // println!(
            //     "number_total = result_1 + result_2  => {} = {} + {}",
            //     number_total, result_1, result_2
            // );

            // let (result_clean, carry_clean) = result_1.overflowing_add(result_2);
            //
            carry = number_total / 10;
            number_total %= 10;

            // println!("number_total = {}", number_total);
            // println!("carry = {}", carry);
            Self::add(&mut option_1, &number_total);
        }

        // let option_final = Self::remove_first_element(option_1);
        Self::remove_first_element(option_1)

        //
        // option_final
    }
}
