pub struct Solution {}
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    #[allow(unused)]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

impl Solution {
    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut count_map: HashMap<i32, i32> = HashMap::new();
        
        fn traverse(node: Option<Rc<RefCell<TreeNode>>>, count_map: &mut HashMap<i32, i32>) {
            if let Some(n) = node {
                let val = n.borrow().val;
                *count_map.entry(val).or_insert(0) += 1;
                traverse(n.borrow().left.clone(), count_map);
                traverse(n.borrow().right.clone(), count_map);
            }
        }
        traverse(root, &mut count_map);
        let max_count = count_map.values().cloned().max().unwrap_or(0);
        let modes: Vec<i32> = count_map
            .iter()
            .filter(|&(_, &count)| count == max_count)
            .map(|(&val, _)| val)
            .collect();
        return modes;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![2],
            Solution::find_mode(Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None
                })))
            }))))
        );
        assert_eq!(
            vec![0],
            Solution::find_mode(Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: None,
                        right: None
                    }))),
                }))),
                right: None
            }))))
        );
    }
}
