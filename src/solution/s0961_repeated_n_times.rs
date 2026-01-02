
pub struct Solution {}

impl Solution {
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        for i in 0..(n-2) {
            if nums[i] == nums[i+1] || nums[i] == nums[i+2] {
                return nums[i];
            }
        }
        nums[n-1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::repeated_n_times(vec![1,2,3,3]));

    }
}
