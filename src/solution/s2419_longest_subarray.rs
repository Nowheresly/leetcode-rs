
pub struct Solution {}

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut max = 0;
        for i in 0..nums.len() {
            max = max.max(nums[i]);
        }
        let mut count = 0;
        let mut ret = 0;
        for i in 0..nums.len() {
            if nums[i] == max {
                count += 1;
            } else {
                ret = ret.max(count);
                count = 0;
            }
        }
        ret = ret.max(count);
        ret
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {

        assert_eq!(2, Solution::longest_subarray(vec![1,2,3,3,2,2]));
        assert_eq!(1, Solution::longest_subarray(vec![1,2,3,4]));
    }
}
