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

pub struct Solution {}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            return None;
        }

        let max = nums.iter().max().unwrap().clone();
        let mut root = TreeNode::new(max);
        let idx = nums.iter().position(|&x| x == max).unwrap();
        root.left = Self::construct_maximum_binary_tree(nums.iter().take(idx).cloned().collect());
        root.right =
            Self::construct_maximum_binary_tree(nums.iter().skip(idx + 1).cloned().collect());
        return Some(Rc::new(RefCell::new(root)));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let node_6 = Rc::new(RefCell::new(TreeNode::new(6)));
        let node_3 = Rc::new(RefCell::new(TreeNode::new(3)));
        let node_5 = Rc::new(RefCell::new(TreeNode::new(5)));
        // null for the left child of 3
        let node_2 = Rc::new(RefCell::new(TreeNode::new(2)));
        let node_0 = Rc::new(RefCell::new(TreeNode::new(0)));
        // null for the right child of 5
        // null for the left child of 2
        // null for the left child of 0
        let node_1 = Rc::new(RefCell::new(TreeNode::new(1)));

        // Root (6)
        // Children of 6: Left=3, Right=5
        node_6.borrow_mut().left = Some(node_3.clone());
        node_6.borrow_mut().right = Some(node_5.clone());

        // Children of 3: Left=null, Right=2
        // node_3.borrow_mut().left = None; // Already None by default
        node_3.borrow_mut().right = Some(node_2.clone());

        // Children of 5: Left=0, Right=null
        node_5.borrow_mut().left = Some(node_0.clone());
        // node_5.borrow_mut().right = None; // Already None by default

        // Children of 2: Left=null, Right=1
        // node_2.borrow_mut().left = None; // Already None by default
        node_2.borrow_mut().right = Some(node_1.clone());

        // Children of 0: Left=null, Right=null (already None by default)
        // Children of 1: Left=null, Right=null (already None by default)

        // The root of our statically built tree is node_6
        let root = Some(node_6);

        assert_eq!(
            root,
            Solution::construct_maximum_binary_tree(vec![3, 2, 1, 6, 0, 5])
        );
    }
}
