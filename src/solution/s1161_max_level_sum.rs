use std::collections::VecDeque;

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
      right: None
    }
  }
}
pub struct Solution {
}


use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut level = 1;
        let mut ql = VecDeque::new();
        ql.push_back(root);
        ql.push_back(None);
        let mut max_sum = i32::MIN;
        let mut sum = 0;
        let mut max_level = 1;
        while !ql.is_empty() {
            let node = ql.pop_front().unwrap();
            if node.is_none() {
                if sum > max_sum {
                    max_sum = sum;
                    max_level = level;
                }
                sum = 0;
                level += 1;
                if !ql.is_empty() {
                    ql.push_back(None);
                }
                continue;
            }
            let node = node.unwrap();
            sum += node.borrow().val;
            if node.borrow().left.is_some() {
                ql.push_back(node.borrow().left.clone());
            }
            if node.borrow().right.is_some() {
                ql.push_back(node.borrow().right.clone());
            }
        }
        max_level
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut root = TreeNode::new(1);
        root.left = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        root.right = Some(Rc::new(RefCell::new(TreeNode::new(0))));

        root.left.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        root.left.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(-8))));
        assert_eq!(2, Solution::max_level_sum(Some(Rc::new(RefCell::new(root)))));

    }
}
