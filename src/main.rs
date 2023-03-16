// cargo watch -w src/main.rs -w in -cqs 'cargo -q run < in > out'
use rust_play::*;
use std::cell::RefCell;
use std::rc::Rc;
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
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
    // let mut v1 = get_space_separated::<i32>();
    let root = TreeNode {
        val: 1,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
            right: None,
        }))),
    };
    println!("{root:#?}");
    let result = inorder_traversal(Some(Rc::new(RefCell::new(root))));
    println!("---------- {result:#?}");
}

pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let root = match root {
        Some(node) => node,
        None => return result,
    };
    if let Some(left) = root.borrow_mut().left.take() {
        result.append(&mut inorder_traversal(Some(left)))
    }
    result.push(root.borrow().val);
    if let Some(right) = root.borrow_mut().right.take() {
        result.append(&mut inorder_traversal(Some(right)))
    }
    result
}
