pub struct Solution {}
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
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

impl Solution {
    pub fn largest_bst_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        
        let ret = Self::helper(&root);
        return ret.2;
    }
    
    pub fn helper(root: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32, i32) {
        if root.is_none() {
            return (i32::MAX, i32::MIN, 0); // min, max, size
        }
        let rootnode = root.as_ref().unwrap().borrow();
        let left = Self::helper(&rootnode.left);
        let right = Self::helper(&rootnode.right);
        if rootnode.val > left.1 && rootnode.val < right.0 {
            let size = left.2 + right.2 + 1;
            return (left.0.min(rootnode.val), right.1.max(rootnode.val), size);
        } else {
            return (i32::MIN, i32::MAX, left.2.max(right.2));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            3,
            Solution::largest_bst_subtree(Some(Rc::new(RefCell::new(TreeNode {
                val: 10,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: None,
                        right: None
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 8,
                        left: None,
                        right: None
                    })))
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 15,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 7,
                        left: None,
                        right: None
                    })))
                })))
            }))))
        );
        assert_eq!(
            2,
            Solution::largest_bst_subtree(Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 2,
                            left: Some(Rc::new(RefCell::new(TreeNode {
                                val: 1,
                                left: None,
                                right: None
                            }))),
                            right: None
                        }))),
                        right: None
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: None,
                        right: None
                    }))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 5,
                        left: None,
                        right: None
                    }))),
                    right: None
                })))
            }))))
        );
    }
}
