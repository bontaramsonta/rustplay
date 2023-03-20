use std::{cell::RefCell, rc::Rc};

// cargo watch -w src/main.rs -w in.txt -cqs 'cargo -q run < in.txt > out.txt'
use rust_play::*;
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
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
    // let n = get_input::<i32>().unwrap();
    // let inorder = get_space_separated::<i32>();
    // let postorder = get_space_separated::<i32>();
    let first = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    })));

    let second = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    })));

    // let result = build_tree(inorder, postorder);
    let result = is_same_tree(first, second);
    println!("---------- {result:#?}");
}

pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (p, q) {
        (None, None) => true,
        (Some(_), None) | (None, Some(_)) => false,
        (Some(p), Some(q)) => {
            let p = p.borrow();
            let q = q.borrow();
            p.val == q.val
                && is_same_tree(p.left.clone(), q.left.clone())
                && is_same_tree(p.right.clone(), q.right.clone())
        }
    }
}
