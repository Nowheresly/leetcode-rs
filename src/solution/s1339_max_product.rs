pub struct Solution {}

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
use std::rc::Rc;
use std::cell::RefCell;
const MOD:i64 = 1_000_000_007;
impl Solution {
    pub fn max_product(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut all_sum = vec![];
        let total_sum = compute_sum(root, &mut all_sum);
        let mut res = 0;
        for sum in all_sum {
           res = res.max(sum * (total_sum - sum));
        }
        (res % MOD) as i32
    }
}

fn compute_sum(root: Option<Rc<RefCell<TreeNode>>>, all_sum: &mut Vec<i64>) -> i64 {
    if root.is_none() {
        return 0;
    }
    let left = compute_sum(root.as_ref().unwrap().borrow().left.clone(), all_sum);
    let right = compute_sum(root.as_ref().unwrap().borrow().right.clone(), all_sum);
    let res = left + right + root.as_ref().unwrap().borrow().val as i64;
    all_sum.push(res);
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut root = TreeNode::new(1);
        root.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        root.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));

        root.left.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        root.left.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(5))));

        root.right.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(6))));

        assert_eq!(110, Solution::max_product(Some(Rc::new(RefCell::new(root)))));
    }
}
