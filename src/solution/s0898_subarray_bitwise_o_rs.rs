use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn subarray_bitwise_o_rs(arr: Vec<i32>) -> i32 {
        let mut set:HashSet<i32> = std::collections::HashSet::new();
        let mut cur = std::collections::HashSet::new();
        for i in arr {
            let mut next = std::collections::HashSet::new();
            next.insert(i);
            for &j in &cur {
                next.insert(i | j);
            }
            set.extend(&next);
            cur = next;
        }
        set.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, Solution::subarray_bitwise_o_rs(vec![0]));
        assert_eq!(3, Solution::subarray_bitwise_o_rs(vec![1, 1, 2]));
        assert_eq!(6, Solution::subarray_bitwise_o_rs(vec![1, 2, 4]));
    }
}
