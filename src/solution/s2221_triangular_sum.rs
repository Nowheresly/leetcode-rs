
pub struct Solution {}

impl Solution {
    pub fn triangular_sum(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut v = nums.clone();
        ts(&mut v, n)
    }
}

fn ts(nums: &mut Vec<i32>, max: usize) -> i32 {
    if max == 0 {
        return nums[0];
    }

    for i in 0..max-1 {
        nums[i] = (nums[i] + nums[i + 1]) % 10;
    }
    ts(nums, max - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(8, Solution::triangular_sum(vec![1,2,3,4,5]));

    }
}
