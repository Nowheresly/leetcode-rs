pub struct Solution {}

impl Solution {
    pub fn count_valid_selections(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut total = 0;
        for i in 0..nums.len() {
            total += nums[i];
        }
        let mut left_sum = 0;
        let mut right_sum = total;
        for i in 0..nums.len() {
            if nums[i] == 0 {
                if left_sum - right_sum >= 0 && left_sum - right_sum <=1 {
                    ans += 1;
                }
                if right_sum - left_sum >= 0 && right_sum - left_sum <=1 {
                    ans += 1;
                }
            } else {
                left_sum += nums[i];
                right_sum -= nums[i];
            }
        }
        ans
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::count_valid_selections(vec![1,0,2,0,3]));

    }
}