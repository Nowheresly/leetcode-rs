pub struct Solution {}

impl Solution {
    pub fn min_pair_sum(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut nums = nums;
        nums.sort();
        let mut res = i32::MIN;
        for i in 0..(n/2) {
            let sum = nums[i] + nums[n-1-i];
            res = res.max(sum);
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(7,  Solution::min_pair_sum(vec![3,5,2,3]));
    }
}
