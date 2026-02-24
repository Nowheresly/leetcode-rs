pub struct Solution {}

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
impl Solution {
    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut root_to_leaf = 0;

        let mut stack = vec![(root, 0)];

        while let Some((node_opt, curr_number)) = stack.pop() {

            if let Some(node) = node_opt {
                let node_ref = node.borrow();

                let next_number = (curr_number << 1) | node_ref.val;

                if node_ref.left.is_none() && node_ref.right.is_none() {
                    root_to_leaf += next_number;
                } else {
                    stack.push((node_ref.right.clone(), next_number));
                    stack.push((node_ref.left.clone(), next_number));
                }
            }
        }

        root_to_leaf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut root = TreeNode::new(1);
        root.left = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        root.left.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        root.left.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(1))));

        root.right = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        root.right.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        root.right.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(1))));

        assert_eq!(22, Solution::sum_root_to_leaf(Some(Rc::new(RefCell::new(root)))));

    }
}
