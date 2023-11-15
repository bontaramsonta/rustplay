use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(
        val: i32,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> Self {
        TreeNode { val, left, right }
    }
}

pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        None => 0,
        Some(node) => {
            let node = node.borrow();
            let left_depth = min_depth(node.left.clone());
            let right_depth = min_depth(node.right.clone());
            1 + if left_depth == 0 || right_depth == 0 {
                left_depth.max(right_depth)
            } else {
                left_depth.min(right_depth)
            }
        }
    }
}

pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
    match root {
        None => target_sum == 0,
        Some(node) => {
            let node = node.borrow();
            let new_target = target_sum - node.val;
            match new_target.ge(&0) {
                true => {
                    has_path_sum(node.left.clone(), new_target)
                        || has_path_sum(node.right.clone(), new_target)
                }
                false => false,
            }
        }
    }
}

fn main() {
    let root = TreeNode::new(
        5,
        Some(Rc::new(RefCell::new(TreeNode::new(
            4,
            Some(Rc::new(RefCell::new(TreeNode::new(
                11,
                Some(Rc::new(RefCell::new(TreeNode::new(7, None, None)))),
                Some(Rc::new(RefCell::new(TreeNode::new(2, None, None)))),
            )))),
            None,
        )))),
        Some(Rc::new(RefCell::new(TreeNode::new(
            8,
            Some(Rc::new(RefCell::new(TreeNode::new(13, None, None)))),
            Some(Rc::new(RefCell::new(TreeNode::new(
                4,
                None,
                Some(Rc::new(RefCell::new(TreeNode::new(1, None, None)))),
            )))),
        )))),
    );
    let result = has_path_sum(Some(Rc::new(RefCell::new(root))), 23);
    println!("result = {:?}", result);
}
