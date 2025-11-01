use std::collections::HashSet;

pub struct Solution {}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

}
impl Solution {
    pub fn modified_list(nums: Vec<i32>, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let set:HashSet<i32> = nums.into_iter().collect();

        let mut root = Box::new(ListNode::new(0));
        root.next = head;
        let mut cur = &mut root;

        while let Some(node) = cur.next.take() {
            if set.contains(&node.val) {
                cur.next = node.next;
            } else {
                cur.next = Some(node);
                cur = cur.next.as_mut().unwrap();
            }
        }
        root.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let head = vec_to_list(&[1,2,3,4,5]);
        let ans = vec_to_list(&[4,5]);

        assert_eq!(ans, Solution::modified_list(vec![1,2,3], head));
    }

    fn vec_to_list(v: &[i32]) -> Option<Box<ListNode>> {
        let mut head: Option<Box<ListNode>> = None;
        for &val in v.iter() {
            let mut node = Box::new(ListNode::new(val));
            node.next = head;
            head = Some(node);
        }
        head
    }
}
