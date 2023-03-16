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
    let inorder = get_space_separated::<i32>();
    let postorder = get_space_separated::<i32>();
    let result = build_tree(inorder, postorder);
    println!("---------- {result:#?}");
}

pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    let root_element = postorder.last()?;
    println!("root {root_element} in{inorder:#?} post{postorder:#?}");
    let root_idx_in_inorder = inorder
        .iter()
        .position(|e| e == root_element)
        .expect("root element not found");
    let left_inorder = &inorder[..root_idx_in_inorder];
    let right_inorder = &inorder[root_idx_in_inorder + 1..];

    let left = if !left_inorder.is_empty() {
        let left_postorder = &postorder[..left_inorder.len()];
        println!("left in{left_inorder:#?} post{left_postorder:#?}");
        build_tree(left_inorder.to_vec(), left_postorder.to_vec())
    } else {
        None
    };
    let right = if !right_inorder.is_empty() {
        dbg!(left_inorder.len());
        let right_postorder = &postorder[left_inorder.len()..(postorder.len() - 1)];
        println!("right in{right_inorder:#?} post{right_postorder:#?}");
        build_tree(right_inorder.to_vec(), right_postorder.to_vec())
    } else {
        None
    };
    Some(Rc::new(RefCell::new(TreeNode {
        val: root_element.to_owned(),
        left,
        right,
    })))
}
