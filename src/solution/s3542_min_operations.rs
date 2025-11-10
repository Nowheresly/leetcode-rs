pub struct Solution {}

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut stack = Vec::new();
        let mut res = 0;
        for &num in nums.iter() {
            while stack.is_empty() == false && stack.last().unwrap() > &num {
                stack.pop();
            }
            if num == 0 {
                continue;
            }
            if stack.is_empty() || &num > stack.last().unwrap() {
                res += 1;
                stack.push(num);
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
        assert_eq!(1, Solution::min_operations(vec![0, 2]));
        assert_eq!(3, Solution::min_operations(vec![3, 1, 2, 1]));
        assert_eq!(4, Solution::min_operations(vec![1, 2, 1, 2, 1, 2]));
    }
}
