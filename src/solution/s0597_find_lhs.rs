use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut freq = HashMap::new();
        for &num in &nums {
            *freq.entry(num).or_insert(0) += 1;
        }
        let mut ret:i32 = 0;
        for (&num, &count) in &freq {
            if let Some(&next_count) = freq.get(&(num + 1)) {
                ret = ret.max(count + next_count);
            }
        }
        return ret;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(5, Solution::find_lhs(vec![1, 3, 2, 2, 5, 2, 3, 7]));
        assert_eq!(2, Solution::find_lhs(vec![1, 2, 3, 4]));
        assert_eq!(0, Solution::find_lhs(vec![1, 1, 1, 1]));
    }
}
