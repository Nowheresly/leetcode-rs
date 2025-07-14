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

pub struct Solution {}

impl Solution {
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        let mut str = String::new();
        let mut cur = head;
        while let Some(node) = cur {
            str.push(char::from(node.val as u8 + b'0'));
            cur = node.next;
        }
        return i32::from_str_radix(&str, 2).unwrap();
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            5,
            Solution::get_decimal_value(Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 0,
                    next: Some(Box::new(ListNode { val: 1, next: None }))
                }))
            })))
        );
    }
}
