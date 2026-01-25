
pub struct Solution {}

impl Solution {
    pub fn minimum_difference(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut res = i32::MAX;
        let k = k as usize;
        for i in 0..(nums.iter().len()-k + 1) {
            let diff = nums[i+k-1] - nums[i];
            res = res.min(diff);
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {

        assert_eq!(2, Solution::minimum_difference(vec![9,4,1,7], 2));

    }
}
