use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn find_final_value(nums: Vec<i32>, original: i32) -> i32 {
        let mut set = HashSet::new();
        for n in nums {
            set.insert(n);
        }
        let mut original = original;
        while set.contains(&original) {
            original *= 2;
        }
        original
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(24, Solution::find_final_value(vec![5,3,6,1,12], 3));

    }
}
