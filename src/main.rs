use std::ops::DerefMut;

// cargo watch -w src/main.rs -w in -cqs 'cargo -q run < in > out'
use rust_play::*;

pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

fn main() {
    let test_cases = get_input::<usize>().unwrap();
    for i in 0..test_cases {
        solve(i);
    }
}

fn solve(_a: usize) {
    println!("CASE: {_a}");
    // let a = get_input::<i32>().unwrap();
    let nums = get_space_separated::<i32>();
    let head = vec_to_linked_list(nums);
    let result_head = something(head);
    print_linked_list(head);
    // println!("---------- {result:#?}");
}

pub fn vec_to_linked_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    for &num in nums.iter().rev() {
        let mut node = ListNode::new(num);
        node.next = head;
        head = Some(Box::new(node));
    }
    head
}

pub fn print_linked_list(head: Option<Box<ListNode>>) -> () {
    let mut tail = head;
    let mut temp: Option<Box<ListNode>> = None;
    while tail.is_some() {
        let mut node = tail.unwrap();
        let val = node.val;
        tail = node.next.take();
        if let Some(ref mut last) = temp {
            last.next = Some(node);
        } else {
            temp = Some(node);
        }
        println!("{val}");
    }
}

pub fn something(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut tail = head.as_mut();
    let mut temp: Option<Box<&mut ListNode>> = None;
    while tail.is_some() {
        let mut node = tail.unwrap();
        let val = node.val;
        tail = node.next.as_mut();
        if let Some(mut last) = temp {
            last.next = Some(Box::new(*node.deref_mut()));
        } else {
            temp = Some(Box::new(node.deref_mut()));
        }
        println!("{val}");
    }
    head
}

// pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//     let mut _c = 0;
//     let mut current = head.as_ref();
//     let mut first_duplicate: Option<&Box<ListNode>> = None;
//     while current.is_some() {
//         println!("{_c}");
//         let node = current.unwrap();
//         let next_node = current.map_or(None, |node| node.next.as_ref()).unwrap();
//         let val = node.val;
//         let next_val = next_node.val;
//         println!("{val} {next_val}");
//         if val == next_val && first_duplicate.is_none() {
//             println!("found duplicate");
//             first_duplicate = current;
//         } else if val != next_val && first_duplicate.is_some() {
//             first_duplicate.unwrap().next = Some(*current.unwrap());
//             first_duplicate = None;
//         } else {
//             println!("no action");
//         }
//         current = node.next.as_ref();
//         _c += 1;
//     }
//     head
// }
