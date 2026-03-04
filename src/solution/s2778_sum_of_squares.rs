pub struct Solution {}

impl Solution {
    pub fn sum_of_squares(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut res = 0;
        for i in 1..(n + 1) {
            if n % i == 0 {
                res += nums[i - 1] * nums[i - 1];
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
        assert_eq!(63, Solution::sum_of_squares(vec![2, 7, 1, 19, 18, 3]));
    }
}
