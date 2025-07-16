pub struct Solution {}

impl Solution {
    pub fn max_score(nums: Vec<i32>) -> i32 {
        let mut ret  = 0;
        let mut max = 0;
        for i in (1..nums.len()).rev() {
            max = max.max(nums[i]);
            ret += max;
        }
        ret
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(16, Solution::max_score(vec![1,5,8]));
        assert_eq!(42, Solution::max_score(vec![4,5,2,8,9,1,3]));
    }
}