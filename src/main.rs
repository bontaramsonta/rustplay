// cargo watch -w src/main.rs -w in.txt -cqs 'cargo -q run < in.txt > out.txt'
use rust_play::*;
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

fn main() {
    let test_cases = get_input::<usize>().unwrap();
    for i in 0..test_cases {
        solve(i);
    }
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
    fn from(v: Vec<i32>) -> Option<Box<ListNode>> {
        let mut current: Option<Box<ListNode>> = None;
        for x in v.iter().rev() {
            let mut node = ListNode::new(*x);
            node.next = current;
            current = Some(Box::new(node))
        }
        current
    }
    #[allow(dead_code)]
    fn delete_duplicate(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }
        let mut h = head;
        let mut current = h.as_mut().unwrap();
        while let Some(next_node) = current.next.as_mut() {
            if current.val == next_node.val {
                current.next = next_node.next.take();
            } else {
                current = current.next.as_mut().unwrap();
            }
        }
        h
    }
    fn merge(first: Option<Box<ListNode>>, second: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        let mut current = &mut dummy;
        let (mut first, mut second) = (first.as_deref(), second.as_deref());

        while let (Some(p1), Some(p2)) = (first, second) {
            if p1.val <= p2.val {
                current.next = Some(Box::new(ListNode::new(p1.val)));
                first = p1.next.as_deref();
            } else {
                current.next = Some(Box::new(ListNode::new(p2.val)));
                second = p2.next.as_deref();
            }
            current = current.next.as_mut().unwrap();
        }

        if let Some(p1) = first {
            current.next = Some(Box::new(p1.clone()));
        }

        if let Some(p2) = second {
            current.next = Some(Box::new(p2.clone()));
        }

        dummy.next
    }
}

fn solve(_test_case: usize) -> () {
    let v1 = get_space_separated::<i32>();
    let v2 = get_space_separated::<i32>();
    println!("{v1:?} {v2:?}");
    let first = ListNode::from(v1);
    let second = ListNode::from(v2);
    // let de_duped = ListNode::delete_duplicate(head);
    let result = ListNode::merge(first, second);
    println!("{result:#?}");
}
