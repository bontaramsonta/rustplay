// cargo watch -w src/main.rs -w in.txt -cqs 'cargo -q run < in.txt > out.txt'
use rust_play::*;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(
        val: i32,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> Self {
        TreeNode { val, left, right }
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
    let root = TreeNode::new(
        1,
        Some(Rc::new(RefCell::new(TreeNode::new(
            2,
            Some(Rc::new(RefCell::new(TreeNode::new(2, None, None)))),
            None,
        )))),
        None,
    );
    // dbg!(root);
    // dbg!(is_symmetric(Some(Rc::new(RefCell::new(root)))));
    dbg!(max_depth(Some(Rc::new(RefCell::new(root)))));
}

fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    if let None = root {
        true
    } else {
        let root = root.unwrap();
        let root = root.borrow();
        is_mirror(root.left.as_ref(), root.right.as_ref())
    }
}

fn is_mirror(t1: Option<&Rc<RefCell<TreeNode>>>, t2: Option<&Rc<RefCell<TreeNode>>>) -> bool {
    match (t1, t2) {
        (None, None) => true,
        (Some(t1), Some(t2)) => {
            let t1 = t1.borrow();
            let t2 = t2.borrow();
            t1.val == t2.val
                && is_mirror(t1.left.as_ref(), t2.right.as_ref())
                && is_mirror(t1.right.as_ref(), t2.left.as_ref())
        }
        _ => false,
    }
}

fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        Some(_) => {
            let root = root.unwrap();
            let root = root.borrow();
            1 + max_depth(root.left.clone()).max(max_depth(root.right.clone()))
        }
        None => 0,
    }
}
