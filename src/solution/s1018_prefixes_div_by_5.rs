
pub struct Solution {}

impl Solution {
    pub fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
        let mut ret = vec![];
        let mut num = 0;
        for i in 0..nums.len() {
            num = (num * 2 + nums[i]) % 5;
            ret.push(num == 0);
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![true, false, false], Solution::prefixes_div_by5(vec![0,1,1]));
        assert_eq!(vec![false, false, false], Solution::prefixes_div_by5(vec![1,1,1]));
    }
}
