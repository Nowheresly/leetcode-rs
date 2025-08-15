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
    pub fn vertical_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![];
        }

        // We use a HashMap to store the nodes by their column index.
        let mut columns: HashMap<i32, Vec<i32>> = HashMap::new();
        // A queue for our breadth-first traversal, storing (node, column) pairs.
        let mut queue = std::collections::VecDeque::new();

        let mut min_col = 0;
        let mut max_col = 0;

        queue.push_back((root.unwrap(), 0));

        while let Some((node, col)) = queue.pop_front() {
            // Get a mutable reference to the vector for this column, or insert a new one if it doesn't exist.
            columns.entry(col).or_insert_with(Vec::new).push(node.borrow().val);

            // Update the min and max column indices.
            min_col = min_col.min(col);
            max_col = max_col.max(col);

            let n_borrowed = node.borrow();

            // Push left child to the queue.
            if let Some(left_node) = &n_borrowed.left {
                queue.push_back((left_node.clone(), col - 1));
            }

            // Push right child to the queue.
            if let Some(right_node) = &n_borrowed.right {
                queue.push_back((right_node.clone(), col + 1));
            }
        }

        let mut result = vec![];
        for i in min_col..=max_col {
            // Get the vector for the current column, or an empty one if it doesn't exist.
            if let Some(column_nodes) = columns.get(&i) {
                result.push(column_nodes.clone());
            } else {
                result.push(vec![]);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {

        assert_eq!(
            vec![vec![4], vec![2, 5], vec![1, 10, 9, 6], vec![3], vec![11]],
            Solution::vertical_order(Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 5,
                            left: None,
                            right: Some(Rc::new(RefCell::new(TreeNode {
                                val: 6,
                                left: None,
                                right: None
                            }))),
                        }))),
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 10,
                        left: None,
                        right: None
                    }))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 9,
                        left: None,
                        right: None
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 11,
                        left: None,
                        right: None
                    }))),
                })))
            }))))
        );
    }
}
