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

fn main() {
    let root = TreeNode::new(
        3,
        Some(Rc::new(RefCell::new(TreeNode::new(9, None, None)))),
        Some(Rc::new(RefCell::new(TreeNode::new(
            20,
            Some(Rc::new(RefCell::new(TreeNode::new(15, None, None)))),
            Some(Rc::new(RefCell::new(TreeNode::new(7, None, None)))),
        )))),
    );
    let result = min_depth(Some(Rc::new(RefCell::new(root))));
    println!("result = {:?}", result);
}
