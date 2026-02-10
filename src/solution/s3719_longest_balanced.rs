use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn longest_balanced(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut res = 0;
        for i in 0..(n - 1) {
            let mut set = HashSet::new();
            set.insert(nums[i]);
            let mut even = 0;
            let mut odd = 0;
            if nums[i] & 1 == 1 {
                odd += 1;
            } else {
                even += 1;
            }
            for j in (i+1)..n {
                if set.contains(&(nums[j])) {
                    if even == odd {
                        res = res.max((j - i + 1) as i32);
                    }
                    continue;
                }
                set.insert(nums[j]);
                if nums[j] & 1 == 1 {
                    odd += 1;
                } else {
                    even += 1;
                }
                if even == odd {
                    res = res.max((j - i + 1) as i32);
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(4, Solution::longest_balanced(vec![2, 5, 4, 3]));
        assert_eq!(5, Solution::longest_balanced(vec![3, 2, 2, 5, 4]));
    }
}
